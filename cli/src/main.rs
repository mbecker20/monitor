#![allow(unused)]

use clap::{arg, Arg, Command};

mod helpers;

use helpers::*;

fn cli() -> Command {
    Command::new("monitor")
        .about("\na cli to set up monitor components, like the periphery client")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("core")
                .about("tools to set up monitor core")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .allow_external_subcommands(true)
                .subcommand(
                    Command::new("gen-config")
                        .about("generate a core config file")
                        .arg(
                            arg!(--host <HOST> "the host to use with oauth redirect url, whatever host the user hits to access monitor. eg 'https://monitor.mogh.tech'")
                                .required(true)
                        )
                        .arg(
                            arg!(--path <PATH> "sets path of generated config file. default is '~/.monitor/core.config.toml'")
                                .required(false)
                        )
                        .arg(
                            arg!(--port <PORT> "sets port core will run on. default is 9000. if running in docker, keep this port as is, set the external port when running core start command")
                                .required(false)
                        )
                        .arg(
                            arg!(--"mongo-uri" <URI> "sets the mongo uri to use. default is 'mongodb://monitor-mongo'")
                                .required(false)
                        )
                        .arg(
                            arg!(--"mongo-db-name" <NAME> "sets the db name to use. default is 'monitor'")
                                .required(false)
                        )
                        .arg(
                            arg!(--"jwt-valid-for" <TIMELENGTH> "sets the length of time jwt stays valid for. default is 1-wk (one week)")
                                .required(false)
                        )
                        .arg(
                            arg!(--"slack-url" <URL> "sets the slack url to use for slack notifications")
                                .required(false)
                        ),
                )
                .subcommand(
                    Command::new("start-mongo")
                        .about("start up a local mongo container for monitor core")
                        .arg(
                            arg!(--yes "used in scripts to skip 'enter to continue' step")
                        )
                        .arg(
                            arg!(--name <NAME> "specify the name of the mongo container. default is monitor-mongo")
                                .required(false)
                        )
                        .arg(
                            arg!(--username <USERNAME> "specify the admin username for mongo. default is mongo with no auth")
                                .required(false)
                        )
                        .arg(
                            arg!(--password <PASSWORD> "specify the admin password for mongo. default is mongo with no auth")
                                .required(false)
                        )
                        .arg(
                            arg!(--port <PORT> "sets port mongo will run on. default is 27017")
                                .required(false)
                        )
                        .arg(
                            arg!(--mount <PATH> "sets the path the mongo data is mounted into. default is ~/.monitor/db")
                                .required(false)
                        )
                        .arg(
                            arg!(--network <NETWORK> "sets docker network of mongo container. default is bridge")
                                .required(false)
                        )
                        .arg(
                            arg!(--restart <RESTART> "sets docker restart mode of mongo container. default is unless-stopped")
                        )
                )
                .subcommand(
                    Command::new("start")
                        .about("start up monitor core in container")
                        .arg(
                            arg!(--yes "used in scripts to skip 'enter to continue' step")
                        )
                        .arg(
                            arg!(--name <NAME> "specify the name of the monitor core container. default is monitor-core")
                        )
                        .arg(
                            arg!(--"config-path" <PATH> "specify the file path to use for config. default is ~/.monitor/core.config.toml")
                                .required(false)
                        )
                        .arg(
                            arg!(--port <PORT> "sets port monitor core will run on. default is 9000")
                                .required(false)
                        )
                        .arg(
                            arg!(--network <NETWORK> "sets docker network of monitor core container. default is bridge")
                                .required(false)
                        )
                        .arg(
                            arg!(--restart <RESTART> "sets docker restart mode of monitor core container. default is unless-stopped")
                        )
                        .arg(
                            arg!(--"add-internal-host" "adds the docker flag '--add-host=host.docker.internal:host-gateway'. default is true")
                        )
                ),
        )
        .subcommand(
            Command::new("periphery")
                .about("tools to set up monitor periphery")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .allow_external_subcommands(true)
                .subcommand(
                    Command::new("gen-config")
                        .about("generate a periphery config file")
                        .arg(
                            arg!(--path <PATH> "sets path of generated config file. default is '~/.monitor/periphery.config.toml'")
                                .required(false)
                        )
                        .arg(
                            arg!(--port <PORT> "sets port periphery will run on. default is 8000. if running in docker, keep this port as is, set the external port when running periphery start command")
                                .required(false)
                        )
                        .arg(
                            arg!(--"stats-polling-rate" <INTERVAL> "sets stats polling rate to control granularity of system stats returned. default is 5-sec. options: 1-sec, 5-sec, 10-sec, 30-sec, 1-min")
                                .required(false)
                        )
                        .arg(
                            arg!(--"allowed-ips" <IPS> "used to only accept requests from known ips. give ips as comma seperated list, like '--allowed_ips 127.0.0.1,10.20.30.43'. default is empty, which will not block any ip.")
                                .required(false)
                        )
                        .arg(
                            arg!(--"repo-dir" <PATH> "if running in container, this should be '/repos'. default is ~/.monitor/repos").required(false)
                        )
                )
                .subcommand(
                    Command::new("start")
                        .about("tools to start periphery as daemon or container")
                        .subcommand(
                            Command::new("systemd")
                                .about("manage periphery with systemd running under current user")
                                .arg(
                                    arg!(--yes "used in scripts to skip 'enter to continue' step")
                                )
                                .arg(
                                    arg!(--install "specify this to install periphery from crates.io")
                                )
                                .arg(
                                    arg!(--"config-path" <PATH> "specify the file path to use for config. default is ~/.monitor/periphery.config.toml")
                                        .required(false)
                                )
                        )
                        .subcommand(
                            Command::new("daemon")
                                .about("start up monitor periphery daemon")
                                .arg(
                                    arg!(--yes "used in scripts to skip 'enter to continue' step")
                                )
                                .arg(
                                    arg!(--install "specify this to install periphery from crates.io")
                                )
                                .arg(
                                    arg!(--"config-path" <PATH> "specify the file path to use for config. default is ~/.monitor/periphery.config.toml")
                                        .required(false)
                                )
                                .arg(
                                    arg!(--stdout <PATH> "specify the file path for periphery to log stdout to. default is ~/.monitor/periphery.log.out")
                                        .required(false)
                                )
                                .arg(
                                    arg!(--stderr <PATH> "specify the file path for periphery to log stderr to. default is ~/.monitor/periphery.log.err")
                                        .required(false)
                                )
                        )
                        // .subcommand(
                        //     Command::new("container")
                        //         .about("start up monitor periphery in docker container")
                        //         .arg(
                        //             arg!(--yes "used in scripts to skip 'enter to continue' step")
                        //         )
                        //         .arg(
                        //             arg!(--name <NAME> "specify the name of the monitor periphery container. default is monitor-periphery")
                        //         )
                        //         .arg(
                        //             arg!(--"config-path" <PATH> "specify the file path to use for config. default is ~/.monitor/periphery.config.toml")
                        //                 .required(false)
                        //         )
                        //         .arg(arg!(--"repo-dir" <PATH> "specify the folder on host to clone repos into. default is ~/.monitor/repos").required(false))
                        //         .arg(
                        //             arg!(--port <PORT> "sets port monitor periphery will run on. default is 8000")
                        //                 .required(false)
                        //         )
                        //         .arg(
                        //             arg!(--network <NETWORK> "sets docker network of monitor periphery container. default is bridge")
                        //                 .required(false)
                        //         )
                        //         .arg(
                        //             arg!(--restart <RESTART> "sets docker restart mode of monitor periphery container. default is unless-stopped")
                        //         )
                        // )
                ),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("core", sub_matches)) => {
            let core_command = sub_matches.subcommand().expect("\n❌ invalid call, should be 'monitor core <gen-config, start-mongo, start> <flags>' ❌\n");
            match core_command {
                ("gen-config", sub_matches) => gen_core_config(sub_matches),
                ("start-mongo", sub_matches) => start_mongo(sub_matches),
                ("start", sub_matches) => start_core(sub_matches),
                _ => {
                    println!("\n❌ invalid call, should be 'monitor core <gen-config, start-mongo, start> <flags>' ❌\n")
                }
            }
        }
        Some(("periphery", sub_matches)) => {
            let periphery_command = sub_matches.subcommand().expect(
                "\n❌ invalid call, should be 'monitor periphery <gen-config, start> <flags>' ❌\n",
            );
            match periphery_command {
                ("gen-config", sub_matches) => gen_periphery_config(sub_matches),
                ("start", sub_matches) => {
                    let periphery_start_command = sub_matches.subcommand().expect("\n❌ invalid call, should be 'monitor periphery start <daemon, container> <flags>' ❌\n");
                    match periphery_start_command {
                        ("systemd", sub_matches) => start_periphery_systemd(sub_matches),
                        ("daemon", sub_matches) => start_periphery_daemon(sub_matches),
                        // ("container", sub_matches) => start_periphery_container(sub_matches),
                        _ => println!("\n❌ invalid call, should be 'monitor periphery start <daemon, container> <flags>' ❌\n")
                    }
                }
                _ => {
                    println!("\n❌ invalid call, should be 'monitor periphery <gen-config, start>...' ❌\n")
                }
            }
        }
        _ => println!("\n❌ invalid call, should be 'monitor <core, periphery> ...' ❌\n"),
    }
}
