use std::env;
use std::collections::HashMap;
use lazy_static::lazy_static;
use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};

lazy_static! {
    static ref PRETTY_NAMES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("cmd", "Command Prompt");
        m.insert("powershell", "PowerShell");
        m.insert("konsole", "Konsole");
        m.insert("gnome-terminal", "GNOME Terminal");
        m.insert("xterm", "XTerm");
        m.insert("xfce4-terminal", "XFCE Terminal");
        m.insert("kitty", "KiTTY");
        m.insert("alacritty", "Alacritty");
        m
    };
}

// Allows detection of shells that host themselves (i.e. Command Prompt).
const SELF_HOSTED_SHELLS: [&str; 2] = [
    "cmd.exe",
    "powershell.exe"
];

macro_rules! env_to_name {
    ($env: expr, $name: expr) => {
        match env::var($env) {
            Ok(_) => return Some($name.to_string()),
            Err(_) => (),
        };
    }
}

pub fn get_terminal() -> Option<String> {
    // Match Apple terminals.
    match env::var("TERM_PROGRAM") {
        Ok(f) => {
            return match f.as_str() {
                "iTerm.app" => Some("iTerm2".to_string()),
                "Terminal.app" => Some("Apple Terminal".to_string()),
                "Hyper.app" => Some("Hyper".to_string()),
                _ => None,
            };
        }
        Err(_) => (),
    };

    env_to_name!("ConEmuPID", "ConEmu");
    env_to_name!("WT_SESSION", "Windows Terminal");
    env_to_name!("SSH_CONNECTION", "SSH");
    env_to_name!("tw52", "TosWin2");
    env_to_name!("tw100", "TosWin2");

    let mut pid = Pid::from_u32(std::process::id());
    let shell = match env::var("SHELL") {
        Ok(f) => f,
        Err(_) => "".to_string(),
    };
    let shell_name = shell.split("/").last().unwrap();

    let mut name: Option<String> = None;

    let mut self_hosted = false;

    // Get to the shell PID.
    'find_shell: loop {
        let ppid = pid_to_ppid(pid);

        if ppid.is_none() { // We ran out of parents.
            return None;
        } else if ppid.unwrap().as_u32() == 1 { // We have reached the daemon.
            return None;
        }

        pid = ppid.unwrap(); // It should be safe to unwrap the PPID now.

        let pid_name = pid_to_name(pid);

        if pid_name.is_none() {
            continue;
        }

        let name_unwrapped = pid_name.unwrap();

        // Detect self-hosted shells.
        for shell in SELF_HOSTED_SHELLS {
            if name_unwrapped == shell {
                self_hosted = true;
                name = Some(name_unwrapped);
                break 'find_shell;
            }
        }

        if name_unwrapped == shell.as_str() || name_unwrapped == shell_name {
            name = Some(name_unwrapped);
            break; // We found the shell.
        }
    }

    if !self_hosted {
        // Try to get parent once more.
        match pid_to_ppid(pid) {
            None => return None,
            Some(f) => {
                // Try to get name.
                name = pid_to_name(f);
            },
        };
    }

    return match name {
        Some(f) => {
            // Remove the file extension.
            let mut res = f.split(".").nth(0).unwrap().to_string();

            // Try to get a "prettier name".
            if PRETTY_NAMES.contains_key(res.as_str()) {
                res = PRETTY_NAMES.get(res.as_str()).unwrap().to_string();
            }

            Some(res)
        },
        None => None,
    }
}

fn pid_to_name(pid: Pid) -> Option<String> {
    let mut system = System::new();
    system.refresh_processes();

    for process in system.processes() {
        if *process.0 == pid {
            return Some(process.1.name().to_string());
        }
    }

    None
}

fn pid_to_ppid(pid: Pid) -> Option<Pid> {
    let mut system = System::new();
    system.refresh_processes();

    for process in system.processes() {
        // Check if the process in question is ours.
        if *process.0 != pid {
            continue;
        }

        // Check if it has a parent.
        if process.1.parent().is_none() {
            continue;
        }

        // Get the parent PID.
        return Some(process.1.parent().unwrap());
    }

    None
}