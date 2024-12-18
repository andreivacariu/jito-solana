#![allow(clippy::arithmetic_side_effects)]
#[cfg(not(any(target_env = "msvc", target_os = "freebsd")))]
use jemallocator::Jemalloc;
use {
    agave_validator::{
        cli::{app, warn_for_deprecated_arguments, DefaultArgs},
        commands,
    },
    solana_streamer::socket::SocketAddrSpace,
    std::path::PathBuf,
};

#[cfg(not(any(target_env = "msvc", target_os = "freebsd")))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

pub fn main() {
    let default_args = DefaultArgs::new();
    let solana_version = solana_version::version!();
    let cli_app = app(solana_version, &default_args);
    let matches = cli_app.get_matches();
    warn_for_deprecated_arguments(&matches);

    let socket_addr_space = SocketAddrSpace::new(matches.is_present("allow_private_addr"));
    let ledger_path = PathBuf::from(matches.value_of("ledger_path").unwrap());

    match matches.subcommand() {
        ("init", _) => {
            commands::run::execute(
                &matches,
                solana_version,
                socket_addr_space,
                &ledger_path,
                commands::run::execute::Operation::Initialize,
            );
        }
        ("", _) | ("run", _) => {
            commands::run::execute(
                &matches,
                solana_version,
                socket_addr_space,
                &ledger_path,
                commands::run::execute::Operation::Run,
            );
        }
        // ("set-block-engine-config", Some(subcommand_matches)) => {
        //     let block_engine_url = value_t_or_exit!(subcommand_matches, "block_engine_url", String);
        //     let trust_packets = subcommand_matches.is_present("trust_block_engine_packets");
        //     let admin_client = admin_rpc_service::connect(&ledger_path);
        //     admin_rpc_service::runtime()
        //         .block_on(async move {
        //             admin_client
        //                 .await?
        //                 .set_block_engine_config(block_engine_url, trust_packets)
        //                 .await
        //         })
        //         .unwrap_or_else(|err| {
        //             println!("set block engine config failed: {}", err);
        //             exit(1);
        //         });
        //     return;
        // }
        // ("set-relayer-config", Some(subcommand_matches)) => {
        //     let relayer_url = value_t_or_exit!(subcommand_matches, "relayer_url", String);
        //     let trust_packets = subcommand_matches.is_present("trust_relayer_packets");
        //     let expected_heartbeat_interval_ms: u64 =
        //         value_of(subcommand_matches, "relayer_expected_heartbeat_interval_ms").unwrap();
        //     let max_failed_heartbeats: u64 =
        //         value_of(subcommand_matches, "relayer_max_failed_heartbeats").unwrap();
        //     let admin_client = admin_rpc_service::connect(&ledger_path);
        //     admin_rpc_service::runtime()
        //         .block_on(async move {
        //             admin_client
        //                 .await?
        //                 .set_relayer_config(
        //                     relayer_url,
        //                     trust_packets,
        //                     expected_heartbeat_interval_ms,
        //                     max_failed_heartbeats,
        //                 )
        //                 .await
        //         })
        //         .unwrap_or_else(|err| {
        //             println!("set relayer config failed: {}", err);
        //             exit(1);
        //         });
        //     return;
        // }
        // ("set-shred-receiver-address", Some(subcommand_matches)) => {
        //     let addr = value_t_or_exit!(subcommand_matches, "shred_receiver_address", String);
        //     let admin_client = admin_rpc_service::connect(&ledger_path);
        //     admin_rpc_service::runtime()
        //         .block_on(async move { admin_client.await?.set_shred_receiver_address(addr).await })
        //         .unwrap_or_else(|err| {
        //             println!("set shred receiver address failed: {}", err);
        //             exit(1);
        //         });
        //     return;
        // }
        // ("set-shred-retransmit-receiver-address", Some(subcommand_matches)) => {
        //     let addr = value_t_or_exit!(subcommand_matches, "shred_receiver_address", String);
        //     let admin_client = admin_rpc_service::connect(&ledger_path);
        //     admin_rpc_service::runtime()
        //         .block_on(async move {
        //             admin_client
        //                 .await?
        //                 .set_shred_retransmit_receiver_address(addr)
        //                 .await
        //         })
        //         .unwrap_or_else(|err| {
        //             println!("set shred receiver address failed: {}", err);
        //             exit(1);
        //         });
        //     return;
        // }
        ("authorized-voter", Some(authorized_voter_subcommand_matches)) => {
            commands::authorized_voter::execute(authorized_voter_subcommand_matches, &ledger_path);
        }
        ("plugin", Some(plugin_subcommand_matches)) => {
            commands::plugin::execute(plugin_subcommand_matches, &ledger_path);
        }
        ("runtime-plugin", Some(plugin_subcommand_matches)) => {
            // let runtime_plugin_rpc_client = runtime_plugin_admin_rpc_service::connect(&ledger_path);
            // let runtime = Runtime::new().unwrap();
            // match plugin_subcommand_matches.subcommand() {
            //     ("list", _) => {
            //         let plugins = runtime
            //             .block_on(
            //                 async move { runtime_plugin_rpc_client.await?.list_plugins().await },
            //             )
            //             .unwrap_or_else(|err| {
            //                 println!("Failed to list plugins: {err}");
            //                 exit(1);
            //             });
            //         if !plugins.is_empty() {
            //             println!("Currently the following plugins are loaded:");
            //             for (plugin, i) in plugins.into_iter().zip(1..) {
            //                 println!("  {i}) {plugin}");
            //             }
            //         } else {
            //             println!("There are currently no plugins loaded");
            //         }
            //         return;
            //     }
            //     ("unload", Some(subcommand_matches)) => {
            //         if let Ok(name) = value_t!(subcommand_matches, "name", String) {
            //             runtime
            //                 .block_on(async {
            //                     runtime_plugin_rpc_client
            //                         .await?
            //                         .unload_plugin(name.clone())
            //                         .await
            //                 })
            //                 .unwrap_or_else(|err| {
            //                     println!("Failed to unload plugin {name}: {err:?}");
            //                     exit(1);
            //                 });
            //             println!("Successfully unloaded plugin: {name}");
            //         }
            //         return;
            //     }
            //     ("load", Some(subcommand_matches)) => {
            //         if let Ok(config) = value_t!(subcommand_matches, "config", String) {
            //             let name = runtime
            //                 .block_on(async {
            //                     runtime_plugin_rpc_client
            //                         .await?
            //                         .load_plugin(config.clone())
            //                         .await
            //                 })
            //                 .unwrap_or_else(|err| {
            //                     println!("Failed to load plugin {config}: {err:?}");
            //                     exit(1);
            //                 });
            //             println!("Successfully loaded plugin: {name}");
            //         }
            //         return;
            //     }
            //     ("reload", Some(subcommand_matches)) => {
            //         if let Ok(name) = value_t!(subcommand_matches, "name", String) {
            //             if let Ok(config) = value_t!(subcommand_matches, "config", String) {
            //                 println!(
            //                     "This command does not work as intended on some systems.\
            //                     To correctly reload an existing plugin make sure to:\
            //                         1. Rename the new plugin binary file.\
            //                         2. Unload the previous version.\
            //                         3. Load the new, renamed binary using the 'Load' command."
            //                 );
            //                 runtime
            //                     .block_on(async {
            //                         runtime_plugin_rpc_client
            //                             .await?
            //                             .reload_plugin(name.clone(), config.clone())
            //                             .await
            //                     })
            //                     .unwrap_or_else(|err| {
            //                         println!("Failed to reload plugin {name}: {err:?}");
            //                         exit(1);
            //                     });
            //                 println!("Successfully reloaded plugin: {name}");
            //             }
            //         }
            //         return;
            //     }
            //     _ => unreachable!(),
            // }
        }
        ("contact-info", Some(subcommand_matches)) => {
            commands::contact_info::execute(subcommand_matches, &ledger_path);
        }
        ("exit", Some(subcommand_matches)) => {
            commands::exit::execute(subcommand_matches, &ledger_path);
        }
        ("monitor", _) => {
            commands::monitor::execute(&matches, &ledger_path);
        }
        ("staked-nodes-overrides", Some(subcommand_matches)) => {
            commands::staked_nodes_overrides::execute(subcommand_matches, &ledger_path);
        }
        ("set-identity", Some(subcommand_matches)) => {
            commands::set_identity::execute(subcommand_matches, &ledger_path);
        }
        ("set-log-filter", Some(subcommand_matches)) => {
            commands::set_log_filter::execute(subcommand_matches, &ledger_path);
        }
        ("wait-for-restart-window", Some(subcommand_matches)) => {
            commands::wait_for_restart_window::execute(subcommand_matches, &ledger_path);
        }
        ("repair-shred-from-peer", Some(subcommand_matches)) => {
            commands::repair_shred_from_peer::execute(subcommand_matches, &ledger_path);
        }
        ("repair-whitelist", Some(repair_whitelist_subcommand_matches)) => {
            commands::repair_whitelist::execute(repair_whitelist_subcommand_matches, &ledger_path);
        }
        ("set-public-address", Some(subcommand_matches)) => {
            commands::set_public_address::execute(subcommand_matches, &ledger_path);
        }
        _ => unreachable!(),
    };
}

// fn tip_manager_config_from_matches(
//     matches: &ArgMatches,
//     voting_disabled: bool,
// ) -> TipManagerConfig {
//     TipManagerConfig {
//         tip_payment_program_id: pubkey_of(matches, "tip_payment_program_pubkey").unwrap_or_else(
//             || {
//                 if !voting_disabled {
//                     panic!("--tip-payment-program-pubkey argument required when validator is voting");
//                 }
//                 Pubkey::new_unique()
//             },
//         ),
//         tip_distribution_program_id: pubkey_of(matches, "tip_distribution_program_pubkey")
//             .unwrap_or_else(|| {
//                 if !voting_disabled {
//                     panic!("--tip-distribution-program-pubkey argument required when validator is voting");
//                 }
//                 Pubkey::new_unique()
//             }),
//         tip_distribution_account_config: TipDistributionAccountConfig {
//             merkle_root_upload_authority: pubkey_of(matches, "merkle_root_upload_authority")
//                 .unwrap_or_else(|| {
//                     if !voting_disabled {
//                         panic!("--merkle-root-upload-authority argument required when validator is voting");
//                     }
//                     Pubkey::new_unique()
//                 }),
//             vote_account: pubkey_of(matches, "vote_account").unwrap_or_else(|| {
//                 if !voting_disabled {
//                     panic!("--vote-account argument required when validator is voting");
//                 }
//                 Pubkey::new_unique()
//             }),
//             commission_bps: value_t!(matches, "commission_bps", u16).unwrap_or_else(|_| {
//                 if !voting_disabled {
//                     panic!("--commission-bps argument required when validator is voting");
//                 }
//                 0
//             }),
//         },
//     }
// }
