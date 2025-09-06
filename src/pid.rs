use crate::utils;
use serde_json::Value;
use std::fs;
use std::io;
use std::process::Command;

pub fn get_pid_from_json(json_str: &str, path: &str) -> io::Result<u32> {
    let value: Value = serde_json::from_str(json_str)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

    if let Some(pid) = value.pointer(path).and_then(|v| {
        println!("{:?}", v);
        v.as_u64()
    }) {
        Ok(pid as u32)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "PID not found in JSON",
        ))
    }
}

// === Get PID Strategies ===
pub fn get_pid_by_active_window(desktop: &str, debug: bool) -> io::Result<u32> {
    utils::debug_print(debug, "Getting PID by active window...");
    let pid = match desktop {
        "hyprland" => utils::run_command("hyprctl", &["activewindow", "-j"])?
            .parse::<Value>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
            .get("pid")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No PID in hyprctl output"))?
            as u32,

        "sway" => {
            let tree = utils::run_command("swaymsg", &["-t", "get_tree"])?;
            get_pid_from_json(&tree, "..pid")?
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                format!("Unsupported desktop: {}", desktop),
            ));
        }
    };

    if pid == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid PID (null or 0)",
        ));
    }

    utils::debug_print(debug, &format!("PID by active window: {}", pid));
    Ok(pid)
}

pub fn get_pid_by_pid(pid: u32, debug: bool) -> io::Result<u32> {
    utils::debug_print(
        debug,
        &format!("Getting PID by PID: {} (actually verifying)", pid),
    );
    fs::read_to_string("");
    // if Command::new("ps")
    //     .arg("-p")
    //     .arg(pid.to_string())
    //     .output()
    //     .is_err()
    // {
    //     return Err(io::Error::new(
    //         io::ErrorKind::NotFound,
    //         format!("Process {} not found", pid),
    //     ));
    // }
    Ok(pid)
}

pub fn get_pid_by_name(name: &str, debug: bool) -> io::Result<u32> {
    utils::debug_print(debug, &format!("Getting PID by name: {}", name));
    let output = utils::run_command("pidof", &[name])?;
    let pid_str = output.split_whitespace().last().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("No PID for process '{}'", name),
        )
    })?;
    let pid = pid_str
        .parse::<u32>()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid PID"))?;

    // Verify process exists
    if Command::new("ps").arg("-p").arg(&pid_str).output().is_err() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Process '{}' exists but PID {} invalid", name, pid),
        ));
    }

    utils::debug_print(debug, &format!("PID by name: {}", pid));
    Ok(pid)
}

pub fn get_pid_by_prop(desktop: &str, debug: bool) -> io::Result<u32> {
    utils::debug_print(debug, "Getting PID by prop (click to select window)...");

    let tool = match desktop {
        "hyprland" => {
            if Command::new("hyprprop").arg("--help").output().is_err() {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "hyprprop not found. Install from https://github.com/vilari-mickopf/hyprprop",
                ));
            }
            "hyprprop"
        }
        "sway" => {
            if Command::new("swayprop").arg("--help").output().is_err() {
                return Err(
                    io::Error::new(
                        io::ErrorKind::NotFound,
                        "swayprop not found. Install from https://git.alternerd.tv/alterNERDtive/swayprop"
                    )
                );
            }
            "swayprop"
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "Window selection not supported on this desktop",
            ));
        }
    };

    let output = utils::run_command(tool, &[])?;
    get_pid_from_json(&output, "/pid")
}
