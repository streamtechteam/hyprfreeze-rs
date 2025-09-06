use clap::Parser;

/// CLI Arguments for hyprfreeze
#[derive(Parser, Debug)]
#[clap(
    name = "hyprfreeze",
    about = "Suspend/resume processes in Hyprland/Sway"
)]
pub struct Args {
    #[clap(short = 'a', long = "active", conflicts_with_all = &["PID", "name", "prop"])]
    pub active: bool,

    #[clap(
        short = 'p',
        long = "pid",
        value_name = "pid",
        conflicts_with_all = &["active", "name", "prop"]
    )]
    pub pid: Option<u32>,

    #[clap(
        short = 'n',
        long = "name",
        value_name = "NAME",
        conflicts_with_all = &["active", "pid", "prop"]
    )]
    pub name: Option<String>,

    #[clap(short = 'r', long = "prop", conflicts_with_all = &["active", "pid", "name"])]
    pub prop: bool,

    #[clap(short = 's', long = "silent")]
    pub silent: bool,

    #[clap(short = 'k', long = "kill")]
    pub kill: bool,

    #[clap(short = 'P', long = "pidof")]
    pub pidof: bool,

    #[clap(short = 't', long = "notif-timeout", default_value = "5000")]
    pub notif_timeout: u32,

    #[clap(long = "info")]
    pub info: bool,

    #[clap(long = "dry-run")]
    pub dry_run: bool,

    #[clap(long = "debug")]
    pub debug: bool,
}
