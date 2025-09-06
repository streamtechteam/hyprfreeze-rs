use crate::process;
use crate::utils;
use std::env;
use std::io::{self, Write};
use std::process::{Command, Stdio};
pub fn debug_print(debug: bool, msg: &str) {
    if debug {
        eprintln!("[DEBUG] {}", msg);
    }
}

pub fn run_command(cmd: &str, args: &[&str]) -> io::Result<String> {
    debug_print(true, &format!("Running: {} {}", cmd, args.join(" ")));
    let output = Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("{}: {}", cmd, err),
        ))
    }
}

pub fn output_pidof(pid: u32, name: &str, debug: bool) {
    println!("{} pid is :{}", name, pid);
    run_command(
        "zenity",
        &[
            "--info",
            format!("--text=\"{} pid is : {}\"", name, pid).as_str(),
        ],
    );
    // Ok(())
}
pub fn send_notification(pid: u32, notif_timeout: u32) -> io::Result<()> {
    let state = process::get_process_state(pid)?;
    let title = if state == "T" {
        format!(
            "Suspended {}",
            process::get_name_by_pid(pid)? // format!("PID {}", pid) // utils::run_command("ps", &["-o", "comm=", "-p", &pid.to_string()])?.trim()
        )
    } else {
        format!(
            "Resumed {}",
            process::get_name_by_pid(pid)? //format!("PID {}", pid) // utils::run_command("ps", &["-o", "comm=", "-p", &pid.to_string()])?.trim()
        )
    };
    let message = format!("PID {}", pid);

    Command::new("notify-send")
        .arg(&title)
        .arg(&message)
        .arg("-t")
        .arg(notif_timeout.to_string())
        .arg("-a")
        .arg("Hyprfreeze")
        .status()?;
    Ok(())
}

pub fn print_info(pid: u32) -> io::Result<()> {
    println!("\x1b[1mProcess tree:\x1b[0m");
    if let Ok(tree) = utils::run_command("pstree", &["-p", &pid.to_string()]) {
        println!("{}", tree);
    }

    println!("\n\x1b[1mProcess threads:\x1b[0m");
    if let Ok(threads) = utils::run_command("ps", &["-eLo", "pid,tid,comm"]) {
        for line in threads.lines() {
            if line.contains(&format!(" {}", pid)) || line.starts_with(&format!("{} ", pid)) {
                println!("{}", line);
            }
        }
    }

    println!("\n\x1b[1mProcess ID\x1b[0m = {}", pid);
    println!(
        "\x1b[1mProcess name\x1b[0m = {}",
        utils::run_command("ps", &["-o", "comm=", "-p", &pid.to_string()])?.trim()
    );
    println!(
        "\x1b[1mProcess state\x1b[0m = {}",
        process::get_process_state(pid)?
    );

    Ok(())
}
