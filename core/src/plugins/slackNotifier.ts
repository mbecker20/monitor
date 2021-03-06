import { getSystemStats } from "@monitor/util-node";
import { Block, KnownBlock } from "@slack/web-api";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import {
  CPU_USAGE_NOTIFY_LIMIT,
  DISK_USAGE_NOTIFY_LIMIT,
  MEM_USAGE_NOTIFY_LIMIT,
  SERVER_STATS_INTERVAL,
  SECRETS,
  CLEAR_ALREADY_ALERTED_INTERVAL,
} from "../config";
import { getPeripherySystemStats } from "../util/periphery/server";
import { serverStatusPeriphery } from "../util/periphery/status";
import {
  notifySlackAdvanced,
  notifySlackCpu,
  notifySlackDisk,
  notifySlackMem,
  notifySlackUnreachable,
} from "../util/slack";

declare module "fastify" {
  interface FastifyInstance {
    dailyInterval: () => Promise<void>;
  }
}

let alreadyAlerted: {
  [serverID: string]: { cpu: boolean; mem: boolean; disk: boolean };
} = {};

let unreachable: string[] = [];

const slackNotifier = fp((app: FastifyInstance, _: {}, done: () => void) => {
  const getAllServerStats = async () => {
    const servers = await app.servers.find({});
    const serversWithStatus = await Promise.all(
      servers
        .filter((server) => server.enabled)
        .map(async (server) => {
          const status = await serverStatusPeriphery(server);
          return {
            ...server,
            stats: server.isCore
              ? await getSystemStats()
              : status
              ? await getPeripherySystemStats(server)
              : undefined,
          };
        })
    );
    return serversWithStatus;
  };

  const interval = async () => {
    const allServers = await getAllServerStats();
    const servers = allServers.filter((server) => server.stats);
    const unreachableServers = allServers.filter(
      (server) => server.stats === undefined
    );
    servers.forEach((server) => {
      // check for out of bounds stats
      const stats = server.stats!;
      if (stats.cpu > (server.cpuAlert || CPU_USAGE_NOTIFY_LIMIT)) {
        // high cpu usage
        if (!alreadyAlerted[server._id!] || !alreadyAlerted[server._id!].cpu) {
          notifySlackCpu(
            server.name,
            server.region!,
            stats.cpu,
            server.toNotify
          );
          if (alreadyAlerted[server._id!]) {
            alreadyAlerted[server._id!] = {
              ...alreadyAlerted[server._id!],
              cpu: true,
            };
          } else {
            alreadyAlerted[server._id!] = {
              cpu: true,
              mem: false,
              disk: false,
            };
          }
        }
      }
      if (
        stats.mem.usedMemPercentage >
        (server.memAlert || MEM_USAGE_NOTIFY_LIMIT)
      ) {
        // high memory usage
        if (!alreadyAlerted[server._id!] || !alreadyAlerted[server._id!].mem) {
          notifySlackMem(
            server.name,
            server.region,
            stats.mem.usedMemMb,
            stats.mem.totalMemMb,
            stats.mem.usedMemPercentage,
            server.toNotify
          );
          if (alreadyAlerted[server._id!]) {
            alreadyAlerted[server._id!] = {
              ...alreadyAlerted[server._id!],
              mem: true,
            };
          } else {
            alreadyAlerted[server._id!] = {
              cpu: false,
              mem: true,
              disk: false,
            };
          }
        }
      }
      if (
        stats.disk.usedPercentage >
        (server.diskAlert || DISK_USAGE_NOTIFY_LIMIT)
      ) {
        // high disk usage
        if (!alreadyAlerted[server._id!] || !alreadyAlerted[server._id!].disk) {
          notifySlackDisk(
            server.name,
            server.region,
            stats.disk.usedGb,
            stats.disk.totalGb,
            stats.disk.usedPercentage,
            server.toNotify
          );
          if (alreadyAlerted[server._id!]) {
            alreadyAlerted[server._id!] = {
              ...alreadyAlerted[server._id!],
              disk: true,
            };
          } else {
            alreadyAlerted[server._id!] = {
              cpu: false,
              mem: false,
              disk: true,
            };
          }
        }
      }
    });
    unreachableServers.forEach((server) => {
      if (!unreachable.includes(server._id!)) {
        unreachable.push(server._id!);
        notifySlackUnreachable(server.name, server.region!, server.toNotify);
      }
    });
  };

  const dailyInterval = async () => {
    const servers = await getAllServerStats();

    const statsBlocks: (Block | KnownBlock)[] = servers
      .map((server) => {
        const stats = server.stats!;
        if (stats) {
          const inWarning =
            stats.cpu > (server.cpuAlert || CPU_USAGE_NOTIFY_LIMIT) ||
            stats.mem.usedMemPercentage >
              (server.memAlert || MEM_USAGE_NOTIFY_LIMIT) ||
            stats.disk.usedPercentage >
              (server.diskAlert || DISK_USAGE_NOTIFY_LIMIT);
          return [
            {
              type: "section",
              text: {
                type: "mrkdwn",
                text: `*${server.name}*${
                  server.region ? ` | ${server.region}` : ""
                } | *${inWarning ? "WARNING" : "OK"}*\nCPU: *${
                  stats.cpu
                }%*\nMEM: *${stats.mem.usedMemPercentage}%* (${
                  stats.mem.usedMemMb
                } MB of ${stats.mem.totalMemMb} MB)\nDISK: *${
                  stats.disk.usedPercentage
                }%* (${stats.disk.usedGb} GB of ${stats.disk.totalGb} GB)`,
              },
            },
            {
              type: "divider",
            },
          ];
        } else {
          return [
            {
              type: "section",
              text: {
                type: "mrkdwn",
                text: `*${server.name}*${
                  server.region ? ` | ${server.region}` : ""
                } | *UNREACHABLE*`,
              },
            },
            {
              type: "divider",
            },
          ];
        }
      })
      .flat();

    notifySlackAdvanced("INFO | daily update", [
      {
        type: "header",
        text: {
          type: "plain_text",
          text: "INFO | daily update",
        },
      },
      { type: "divider" },
      ...statsBlocks,
    ]);
  };

  app.decorate("dailyInterval", dailyInterval);

  if (SECRETS.SLACK_TOKEN) {
    setInterval(interval, SERVER_STATS_INTERVAL);
    setInterval(dailyInterval, 24 * 60 * 60 * 1000);
    setInterval(() => {
      alreadyAlerted = {};
      unreachable = [];
    }, CLEAR_ALREADY_ALERTED_INTERVAL);
  }

  done();
});

export default slackNotifier;
