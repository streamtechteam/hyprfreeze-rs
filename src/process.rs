use crate::utils;
use std::fs;
use std::io;
use std::process::Command;

// === Process Control ===
pub fn get_process_tree(pid: u32) -> io::Result<Vec<u32>> {
    let output = utils::run_command("pstree", &["-p", &pid.to_string()])?;
    let mut pids = Vec::new();

    // Use regex to capture the number inside parentheses: (\d+)
    for cap in regex::Regex::new(r"\((\d+)\)")
        .unwrap()
        .captures_iter(&output)
    {
        let pid_str = &cap[1]; // Extract first capture group (the number)
        if let Ok(p) = pid_str.parse::<u32>() {
            pids.push(p);
        }
    }

    // Ensure the main PID is included (sometimes not listed in tree)
    if !pids.contains(&pid) {
        pids.push(pid);
    }

    Ok(pids)
}

pub fn get_process_state(pid: u32) -> io::Result<String> {
    let state = utils::run_command("ps", &["-o", "state=", "-p", &pid.to_string()])?;
    Ok(state.trim().to_string())
}

pub fn toggle_suspend(pid: u32, dry_run: bool, debug: bool) -> io::Result<()> {
    let script_pid = std::process::id();
    let pids = get_process_tree(pid)?;

    // Prevent self-freezing
    if pids.contains(&script_pid) {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Cannot suspend hyprfreeze itself",
        ));
    }

    let current_state = get_process_state(pid)?;
    let signal = if current_state == "T" { "CONT" } else { "STOP" };
    let action = if signal == "CONT" {
        "Resumed"
    } else {
        "Suspended"
    };

    utils::debug_print(
        debug,
        &format!("Process state: {}, sending SIG{}", current_state, signal),
    );

    if dry_run {
        utils::debug_print(
            debug,
            &format!("[DRY RUN] Would send SIG{} to PIDs: {:?}", signal, pids),
        );
        println!(
            "[DRY RUN] {} process (PID {}) with {} threads",
            action,
            pid,
            pids.len()
        );
        return Ok(());
    }

    for &p in &pids {
        let status = Command::new("kill")
            .arg(format!("-{}", signal))
            .arg(p.to_string())
            .status()?;
        if !status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to signal PID {}", p),
            ));
        }
    }

    let name = utils::run_command("ps", &["-o", "comm=", "-p", &pid.to_string()])?;
    println!("{} {} (PID {})", action, name.trim(), pid);
    Ok(())
}

pub fn get_name_by_pid(pid: u32) -> io::Result<String> {
    // let name = utils::run_command("ps", &["-o", "comm=", "-p", &pid.to_string()])?;

    let name =
        fs::read_to_string(format!("/proc/{}/comm", pid)).expect("failed to get name by pid ");
    Ok(name)
}

pub fn kill_by_pid(pid: u32, debug: bool) -> io::Result<()> {
    utils::debug_print(debug, &format!("Killing PID: {}", pid));
    utils::run_command("kill", &["-9", &pid.to_string()])?;
    Ok(())
}
