# hyprfreeze-rs

A Rust implementation of [hyprfreeze](https://github.com/Zerodya/hyprfreeze), a utility for suspending and resuming processes in Hyprland and Sway window managers (others will be supported in the future).

## Description

hyprfreeze-rs allows you to suspend and resume processes on demand, which is particularly useful for:
- Freezing resource-intensive applications when not in use
- Suspending games or video players to free up system resources
- Pausing applications without closing them

This Rust version provides the same functionality as the original bash script but with improved performance and reliability.

## Features

- Suspend/resume any process by PID, name, or window selection
- Support for Hyprland and Sway window managers
- Dry-run mode to preview actions
- Notification system to confirm actions
- Process tree handling (suspend/resume all child processes)
- Kill processes with a single command
- Display process information and status

## Installation

### From Source

1. Clone the repository:
```bash
git clone https://github.com/streamtechteam/hyprfreeze-rs.git
cd hyprfreeze-rs
```

2. Build the project:
```bash
cargo build --release
```

3. The binary will be available at `target/release`

### Dependencies

- `pstree` - For process tree visualization
- `kill` - For sending signals to processes
- `ps` - For process information
- `pidof` - For finding process IDs by name
- For Hyprland: `hyprctl` and optionally `hyprprop` (only for selecting window functionality)
- For Sway: `swaymsg` and optionally `swayprop` (only for selecting window functionality)
- `notify-send` - For desktop notifications
- `zenity` - For GUI dialogs

### Download Binary

you can download the latest binary from the [Actions](https://github.com/streamtechteam/hyprfreeze-rs/actions) tab.

## Usage

### Basic Commands

```bash
# Suspend/resume the active window
hyprfreeze-rs -a

# Suspend/resume a process by PID
hyprfreeze-rs -p 1234

# Suspend/resume a process by name
hyprfreeze-rs -n firefox

# Suspend/resume a process by clicking on its window
hyprfreeze-rs -r
```

### Additional Options

```bash
# Kill a process instead of suspending/resuming
hyprfreeze-rs -a -k

# Show process information
hyprfreeze-rs -a --info

# Get PID of a process without taking action
hyprfreeze-rs -n firefox -P

# Dry run mode (preview actions without executing)
hyprfreeze-rs -a --dry-run

# Silent mode (no notifications)
hyprfreeze-rs -a -s

# Debug mode (show detailed output)
hyprfreeze-rs -a --debug

# Set notification timeout (in milliseconds)
hyprfreeze-rs -a -t 3000
```

### Key Bindings (Hyprland example)

Add to your `hyprland.conf`:
```ini
bind = $mod, space, exec, hyprfreeze-rs -a
bind = $mod SHIFT, space, exec, hyprfreeze-rs -a --info
```

## How It Works

hyprfreeze-rs uses POSIX signals to control processes:
- `SIGSTOP` to suspend a process
- `SIGCONT` to resume a suspended process

When you target a process, hyprfreeze-rs:
1. Identifies the process and all its child processes
2. Checks the current state of the process
3. Sends the appropriate signal (STOP or CONT) to all processes in the tree
4. Displays a notification confirming the action

## Desktop Support

### Hyprland

- Active window detection with `hyprctl`
- Window selection with `hyprprop` (optional , required for window selection functionality)

### Sway

- Active window detection with `swaymsg`
- Window selection with `swayprop` (optional , required for window selection functionality)

others will be supported in the future.

## Future Plans

- Support for additional window managers
- Support for additional desktop environments
- Additional features

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- This project is inspired by and based on [hyprfreeze](https://github.com/Zerodya/hyprfreeze) by [Zerodya](https://github.com/Zerodya/)
- Thanks to the Hyprland and Sway communities for their excellent window managers