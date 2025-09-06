use clap::Parser;

use std::clone;
use std::io::{self};
use std::process::exit;

mod args_structure;
mod desktop;
mod pid;
mod process;
mod utils;

// === Main ===
fn main() -> io::Result<()> {
    let args = args_structure::Args::parse();

    if args.debug {
        utils::debug_print(true, &format!("Args: {:?}", args));
    }

    let desktop = desktop::get_desktop_environment();
    utils::debug_print(args.debug, &format!("Detected desktop: {}", desktop));

    // Normalize desktop (e.g. uwsm)
    let desktop = desktop::normilize_desktop(desktop.as_str());

    // let pid = if args.active {
    //     let mut result;
    //     result = pid::get_pid_by_active_window(desktop, args.debug)?;
    //     if args.kill {
    //         process::kill_by_pid(
    //             pid::get_pid_by_active_window(desktop, args.debug).unwrap_or(0),
    //             args.debug,
    //         )
    //         .expect_err("Failed to kill active window");
    //         result = 0;
    //     }
    //     if args.pidof {
    //         let pid = pid::get_pid_by_active_window(desktop, args.debug).unwrap_or(0);
    //         utils::output_pidof(
    //             pid,
    //             process::get_name_by_pid(pid)
    //                 .unwrap_or("error".to_string())
    //                 .as_str(),
    //             args.debug,
    //         );
    //         result = 0;
    //     }
    //     result
    // } else if let Some(pid) = args.pid {
    //     let mut result: u32;
    //     if args.kill {
    //         process::kill_by_pid(pid, args.debug)?;
    //         result = 0
    //     }
    //     result = pid::get_pid_by_pid(pid, args.debug)?;
    //     result
    // } else if let Some(ref name) = args.name {
    //     let mut result: u32;
    //     if args.kill {
    //         process::kill_by_pid(
    //             pid::get_pid_by_name(&name, args.debug).unwrap_or(0),
    //             args.debug,
    //         )
    //         .expect_err("Failed to kill process by name");
    //         // return Ok(());
    //         result = 0;
    //     }
    //     if args.pidof {
    //         let pid = pid::get_pid_by_name(name, args.debug).unwrap_or(0);
    //         utils::output_pidof(
    //             pid,
    //             process::get_name_by_pid(pid)
    //                 .unwrap_or("error".to_string())
    //                 .as_str(),
    //             args.debug,
    //         );
    //         // return Ok(());
    //         result = 0;
    //     }
    //     result = pid::get_pid_by_name(name, args.debug)?;
    //     result
    // } else if args.prop {
    //     let mut result: u32;
    //     let pid = pid::get_pid_by_prop(desktop, args.debug)?;
    //     result = pid.clone();
    //     if args.kill {
    //         process::kill_by_pid(pid, args.debug).expect_err("Failed to kill process by prop");
    //         // return Ok(());
    //         result = 0;
    //     }
    //     if args.pidof {
    //         // let pid = get_pid_by_prop(desktop, args.debug).unwrap_or(0);
    //         utils::output_pidof(
    //             pid,
    //             process::get_name_by_pid(pid)
    //                 .unwrap_or("error".to_string())
    //                 .as_str(),
    //             args.debug,
    //         );
    //         result = 0;
    //         // exit(0)
    //     }
    //     result
    // } else {
    //     eprintln!("Error: One of -a, -p, -n, or -r must be specified.");
    //     std::process::exit(1);
    // };
    //
    //
    let pid_result: io::Result<u32> = if args.active {
        pid::get_pid_by_active_window(desktop, args.debug)
    } else if let Some(pid_val) = args.pid {
        pid::get_pid_by_pid(pid_val, args.debug)
    } else if let Some(ref name) = args.name {
        pid::get_pid_by_name(name, args.debug)
    } else if args.prop {
        pid::get_pid_by_prop(desktop, args.debug)
    } else {
        eprintln!("Error: One of -a, -p, -n, or -r must be specified.");
        exit(1); // Use process::exit
    };
    // let pid = pid_result?;
    // println!("{pid}");
    // Perform suspend/resume
    //
    let pid = match pid_result {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error obtaining PID: {}", e);
            exit(1);
        }
    };

    // Handle kill and pidof *after* getting the PID
    if args.kill {
        if let Err(e) = process::kill_by_pid(pid, args.debug) {
            eprintln!("Error killing PID {}: {}", pid, e);
            exit(1);
        }
        // Exit after killing
        return Ok(());
    }

    if args.pidof {
        let name = process::get_name_by_pid(pid).unwrap_or_else(|_| "unknown".to_string()); // Handle error gracefully for display
        utils::output_pidof(pid, &name, args.debug);
        // Exit after displaying pidof
        return Ok(());
    }
    if (pid != 0) {
        process::toggle_suspend(pid, args.dry_run, args.debug)?;
    }

    // Optional info
    if args.info {
        utils::print_info(pid)?;
    }

    // Send notification unless silent
    if !args.silent && !args.dry_run {
        utils::send_notification(pid, args.notif_timeout)?;
    }

    Ok(())
}
