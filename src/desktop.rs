use crate::utils;
use std::env;

// mod desktop {
pub fn get_desktop_environment() -> String {
    // Try loginctl first
    if let Ok(session) = utils::run_command("loginctl", &["show-session", "auto", "-p", "Desktop"])
    {
        return session.trim().trim_start_matches("Desktop=").to_lowercase();
    }

    // Fallback to XDG_SESSION_DESKTOP
    if let Ok(desktop) = env::var("XDG_SESSION_DESKTOP") {
        return desktop.to_lowercase();
    }

    // Default fallback
    "hyprland".to_string()
}
// }
pub fn normilize_desktop(desktop: &str) -> &str {
    if desktop.ends_with("-uwsm") {
        &desktop[..desktop.len() - 5]
    } else {
        &desktop
    }
}
