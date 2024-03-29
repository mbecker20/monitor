use std::{cmp::Ordering, collections::HashMap, path::PathBuf};

use async_timing_util::{
    unix_timestamp_ms, wait_until_timelength, Timelength, ONE_DAY_MS, ONE_HOUR_MS,
};
use futures_util::future::join_all;
use mungos::mongodb::bson::doc;
use slack::types::Block;
use types::{Server, SystemStats, SystemStatsQuery, SystemStatsRecord};

use crate::state::State;

#[derive(Default, Clone)]
pub struct AlertStatus {
    cpu_alert: bool,
    mem_alert: bool,
    disk_alert: HashMap<PathBuf, bool>,
    component_alert: bool,
}

impl State {
    pub async fn collect_server_stats(&self) {
        loop {
            let ts = wait_until_timelength(
                self.config.monitoring_interval.to_string().parse().unwrap(),
                0,
            )
            .await as i64;
            let servers = self.get_enabled_servers_with_stats().await;
            if let Err(e) = servers {
                eprintln!("failed to get server list from db: {e:?}");
                continue;
            }
            for (server, res) in servers.unwrap() {
                if let Err(_) = res {
                    if let Some(slack) = &self.slack {
                        let (header, info) = generate_unreachable_message(&server);
                        let res = slack.send_message_with_header(&header, info.clone()).await;
                        if let Err(e) = res {
                            eprintln!("failed to send message to slack: {e} | header: {header} | info: {info:?}")
                        }
                    }
                    continue;
                }
                let stats = res.unwrap();
                self.check_server_stats(&server, &stats).await;
                let res = self
                    .db
                    .stats
                    .create_one(SystemStatsRecord::from_stats(server.id, ts, stats))
                    .await;
                if let Err(e) = res {
                    eprintln!("failed to insert stats into mongo | {e}");
                }
            }
        }
    }

    pub async fn prune_stats_on_mongo(&self) {
        let days_ago_ms = self.config.keep_stats_for_days as u128 * ONE_DAY_MS;
        loop {
            let ts = wait_until_timelength(Timelength::OneDay, 0).await;
            let delete_before_ts = ts - days_ago_ms;
            let res = self
                .db
                .stats
                .delete_many(doc! { "ts": { "$lte": delete_before_ts as i64 } })
                .await;
            if let Err(e) = res {
                eprintln!("{ts} | failed to delete old stats | {e:?}");
            }
        }
    }

    async fn get_enabled_servers_with_stats(
        &self,
    ) -> anyhow::Result<Vec<(Server, anyhow::Result<SystemStats>)>> {
        let servers = self
            .db
            .servers
            .get_some(doc! { "enabled": true }, None)
            .await?;

        let futures = servers.into_iter().map(|server| async move {
            let stats = self
                .periphery
                .get_system_stats(&server, &SystemStatsQuery::all())
                .await;
            (server, stats)
        });

        Ok(join_all(futures).await)
    }

    async fn check_server_stats(&self, server: &Server, stats: &SystemStats) {
        self.check_cpu(server, stats).await;
        self.check_mem(server, stats).await;
        self.check_disk(server, stats).await;
        self.check_components(server, stats).await;
    }

    async fn check_cpu(&self, server: &Server, stats: &SystemStats) {
        if self.slack.is_none()
            || self
                .server_alert_status
                .get(&server.id)
                .await
                .map(|s| s.cpu_alert)
                .unwrap_or(false)
        {
            return;
        }
        if stats.cpu_perc > server.cpu_alert {
            let region = if let Some(region) = &server.region {
                format!(" ({region})")
            } else {
                String::new()
            };
            let mut top_procs = stats.processes.clone();
            top_procs.sort_by(|a, b| {
                if a.cpu_perc > b.cpu_perc {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            let top_procs = top_procs
                .into_iter()
                .take(3)
                .enumerate()
                .map(|(i, p)| {
                    format!(
                        "\n{}. *{}* | *{:.1}%* CPU | *{:.1} GiB* MEM",
                        i + 1,
                        p.name,
                        p.cpu_perc,
                        p.mem_mb / 1024.0,
                    )
                })
                .collect::<Vec<_>>()
                .join("");
            let mut blocks = vec![
                Block::header("WARNING 🚨"),
                Block::section(format!(
                    "*{}*{region} has high *CPU usage* 📈 🚨",
                    server.name
                )),
                Block::section(format!("cpu: *{:.1}%*", stats.cpu_perc)),
                Block::section(format!("*top cpu processes*{top_procs}",)),
            ];

            if let Some(to_notify) = generate_to_notify(server) {
                blocks.push(Block::section(to_notify))
            }

            let res = self
                .slack
                .as_ref()
                .unwrap()
                .send_message(
                    format!(
                        "WARNING 🚨 | *{}*{region} has high *CPU usage* 📈 🚨",
                        server.name
                    ),
                    blocks,
                )
                .await;
            if let Err(e) = res {
                eprintln!(
                    "failed to send message to slack | high cpu usage on {} | usage: {:.1}% | {e:?}",
                    server.name, stats.cpu_perc
                )
            } else {
                self.server_alert_status
                    .update_entry(server.id.clone(), |entry| {
                        entry.cpu_alert = true;
                    })
                    .await;
            }
        }
    }

    async fn check_mem(&self, server: &Server, stats: &SystemStats) {
        if self.slack.is_none()
            || self
                .server_alert_status
                .get(&server.id)
                .await
                .map(|s| s.mem_alert)
                .unwrap_or(false)
        {
            return;
        }
        let usage_perc = (stats.mem_used_gb / stats.mem_total_gb) * 100.0;
        if usage_perc > server.mem_alert {
            let region = if let Some(region) = &server.region {
                format!(" ({region})")
            } else {
                String::new()
            };
            let mut top_procs = stats.processes.clone();
            top_procs.sort_by(|a, b| {
                if a.mem_mb > b.mem_mb {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            let top_procs = top_procs
                .into_iter()
                .take(3)
                .enumerate()
                .map(|(i, p)| {
                    format!(
                        "\n{}. *{}* | *{:.1}%* CPU | *{:.1} GiB* MEM",
                        i + 1,
                        p.name,
                        p.cpu_perc,
                        p.mem_mb / 1024.0,
                    )
                })
                .collect::<Vec<_>>()
                .join("");
            let mut blocks = vec![
                Block::header("WARNING 🚨"),
                Block::section(format!(
                    "*{}*{region} has high *memory usage* 💾 🚨",
                    server.name
                )),
                Block::section(format!(
                    "memory: used *{:.2} GB* of *{:.2} GB* (*{:.1}%*)",
                    stats.mem_used_gb, stats.mem_total_gb, usage_perc
                )),
                Block::section(format!("*top mem processes*{top_procs}",)),
            ];

            if let Some(to_notify) = generate_to_notify(server) {
                blocks.push(Block::section(to_notify))
            }

            let res = self
                .slack
                .as_ref()
                .unwrap()
                .send_message(
                    format!(
                        "WARNING 🚨 | *{}*{region} has high *memory usage* 💾 🚨",
                        server.name
                    ),
                    blocks,
                )
                .await;
            if let Err(e) = res {
                eprintln!(
                    "failed to send message to slack | high mem usage on {} | usage: {:.2}GB of {:.2}GB | {e:?}",
                    server.name, stats.mem_used_gb, stats.mem_total_gb,
                )
            } else {
                self.server_alert_status
                    .update_entry(server.id.clone(), |entry| {
                        entry.mem_alert = true;
                    })
                    .await;
            }
        }
    }

    async fn check_disk(&self, server: &Server, stats: &SystemStats) {
        for disk in &stats.disk.disks {
            if self.slack.is_none()
                || self
                    .server_alert_status
                    .get(&server.id)
                    .await
                    .map(|s| *s.disk_alert.get(&disk.mount).unwrap_or(&false))
                    .unwrap_or(false)
            {
                return;
            }
            let usage_perc = (disk.used_gb / disk.total_gb) * 100.0;
            if usage_perc > server.disk_alert {
                let region = if let Some(region) = &server.region {
                    format!(" ({region})")
                } else {
                    String::new()
                };
                let mut blocks = vec![
                    Block::header("WARNING 🚨"),
                    Block::section(format!(
                        "*{}*{region} has high *disk usage* (mount point *{}*) 💿 🚨",
                        server.name,
                        disk.mount.display()
                    )),
                    Block::section(format!(
                        "disk: used *{:.2} GB* of *{:.2} GB* (*{:.1}%*)",
                        disk.used_gb, disk.total_gb, usage_perc
                    )),
                ];

                if let Some(to_notify) = generate_to_notify(server) {
                    blocks.push(Block::section(to_notify))
                }

                let res = self
                    .slack
                    .as_ref()
                    .unwrap()
                    .send_message(
                        format!(
                            "WARNING 🚨 | *{}*{region} has high *disk usage* 💿 🚨",
                            server.name
                        ),
                        blocks,
                    )
                    .await;
                if let Err(e) = res {
                    eprintln!(
                    "failed to send message to slack | high disk usage on {} | usage: {:.2}GB of {:.2}GB | {e:?}",
                    server.name, stats.disk.used_gb, stats.disk.total_gb,
                )
                } else {
                    self.server_alert_status
                        .update_entry(server.id.clone(), |entry| {
                            entry.disk_alert.insert(disk.mount.clone(), true);
                        })
                        .await;
                }
            }
        }
    }

    async fn check_components(&self, server: &Server, stats: &SystemStats) {
        if self.slack.is_none()
            || self
                .server_alert_status
                .get(&server.id)
                .await
                .map(|s| s.component_alert)
                .unwrap_or(false)
        {
            return;
        }
        let info = stats
            .components
            .iter()
            .map(|c| {
                if let Some(critical) = c.critical {
                    if c.temp / critical > 0.85 {
                        format!(
                            "{}: *{:.1}°* (*{:.1}%* to critical) 🌡️",
                            c.label,
                            c.temp,
                            (c.temp / critical) * 100.0
                        )
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                }
            })
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        if info.len() > 0 {
            let region = if let Some(region) = &server.region {
                format!(" ({region})")
            } else {
                String::new()
            };
            let mut blocks = vec![
                Block::header("WARNING 🚨"),
                Block::section(format!(
                    "*{}*{region} has high *tempurature* 🌡️ 🚨",
                    server.name
                )),
                Block::section(info.join("\n")),
            ];

            if let Some(to_notify) = generate_to_notify(server) {
                blocks.push(Block::section(to_notify))
            }

            let res = self
                .slack
                .as_ref()
                .unwrap()
                .send_message(
                    format!(
                        "WARNING 🚨 | *{}*{region} has high *tempurature* 🌡️ 🚨",
                        server.name
                    ),
                    blocks,
                )
                .await;
            if let Err(e) = res {
                eprintln!(
                    "failed to send message to slack | high tempurature on {} | {} | {e:?}",
                    server.name,
                    info.join(" | "),
                )
            } else {
                self.server_alert_status
                    .update_entry(server.id.clone(), |entry| {
                        entry.component_alert = true;
                    })
                    .await;
            }
        }
    }

    pub async fn daily_update(&self) {
        let offset = self.config.daily_offset_hours as u128 * ONE_HOUR_MS;
        loop {
            wait_until_timelength(Timelength::OneDay, offset).await;
            let servers = self.get_enabled_servers_with_stats().await;
            if let Err(e) = &servers {
                eprintln!(
                    "{} | failed to get servers with stats for daily update | {e:#?}",
                    unix_timestamp_ms()
                );
                continue;
            }
            let servers = servers.unwrap();
            if servers.is_empty() {
                continue;
            }
            let mut blocks = vec![Block::header("INFO | daily update"), Block::divider()];
            for (server, stats) in servers {
                let region = if let Some(region) = &server.region {
                    format!(" | {region}")
                } else {
                    String::new()
                };
                if let Ok(stats) = stats {
                    let cpu_warning = if stats.cpu_perc > server.cpu_alert {
                        " 🚨"
                    } else {
                        ""
                    };
                    let mem_warning =
                        if (stats.mem_used_gb / stats.mem_total_gb) * 100.0 > server.mem_alert {
                            " 🚨"
                        } else {
                            ""
                        };
                    let disk_warning =
                        if (stats.disk.used_gb / stats.disk.total_gb) * 100.0 > server.disk_alert {
                            " 🚨"
                        } else {
                            ""
                        };
                    let status = if !cpu_warning.is_empty()
                        || !mem_warning.is_empty()
                        || !disk_warning.is_empty()
                    {
                        "*WARNING* 🚨"
                    } else {
                        "*OK* ✅"
                    };
                    let name_line = format!("*{}*{region} | {status}", server.name);
                    let cpu_line = format!("CPU: *{:.1}%*{cpu_warning}", stats.cpu_perc);
                    let mem_line = format!(
                        "MEM: *{:.1}%* ({:.2} GB of {:.2} GB){mem_warning}",
                        (stats.mem_used_gb / stats.mem_total_gb) * 100.0,
                        stats.mem_used_gb,
                        stats.mem_total_gb,
                    );
                    let disk_line = format!(
                        "DISK: *{:.1}%* ({:.2} GB of {:.2} GB){disk_warning}",
                        (stats.disk.used_gb / stats.disk.total_gb) * 100.0,
                        stats.disk.used_gb,
                        stats.disk.total_gb,
                    );
                    blocks.push(Block::section(format!(
                        "{name_line}\n{cpu_line}\n{mem_line}\n{disk_line}",
                    )));
                } else {
                    blocks.push(Block::section(format!(
                        "*{}*{region} | *UNREACHABLE* ❌",
                        server.name
                    )));
                }
                blocks.push(Block::divider())
            }
            let res = self
                .slack
                .as_ref()
                .unwrap()
                .send_message(format!("INFO | daily update"), blocks)
                .await;
            if let Err(e) = res {
                eprintln!(
                    "{} | failed to send daily update message | {e:?}",
                    unix_timestamp_ms()
                );
            }
            {
                self.server_alert_status.clear().await;
            }
        }
    }
}

fn generate_unreachable_message(server: &Server) -> (String, Option<String>) {
    let region = match &server.region {
        Some(region) => format!(" ({region})"),
        None => String::new(),
    };
    let header = format!("WARNING 🚨 | {}{region} is unreachable ❌", server.name);
    let to_notify = server
        .to_notify
        .iter()
        .map(|u| format!("<@{u}>"))
        .collect::<Vec<_>>()
        .join(" ");
    let info = if to_notify.len() > 0 {
        Some(to_notify)
    } else {
        None
    };
    (header, info)
}

fn generate_to_notify(server: &Server) -> Option<String> {
    if server.to_notify.len() > 0 {
        Some(
            server
                .to_notify
                .iter()
                .map(|u| format!("<@{u}>"))
                .collect::<Vec<String>>()
                .join(" "),
        )
    } else {
        None
    }
}
