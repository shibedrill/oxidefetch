use std::env;
use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};

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

    env_to_name!("WT_SESSION", "Windows Terminal");
    env_to_name!("SSH_CONNECTION", "SSH");
    env_to_name!("tw52", "TosWin2");
    env_to_name!("tw100", "TosWin2");

    let mut pid = Pid::from_u32(std::process::id());
    let shell = match env::var("SHELL") {
        Ok(f) => f,
        Err(_) => return None,
    };
    let shell_name = shell.split("/").last().unwrap();

    // Get to the shell PID.
    loop {
        let ppid = pid_to_ppid(pid);

        if ppid.is_none() { // We ran out of parents.
            return None;
        } else if ppid.unwrap().as_u32() == 1 { // We have reached the daemon.
            return None;
        }

        pid = ppid.unwrap(); // It should be safe to unwrap the PPID now.

        let name = pid_to_name(pid);

        if name.is_none() {
            continue;
        }

        println!("{}", name.as_ref().unwrap());

        let name_unwrapped = name.unwrap();

        if name_unwrapped == shell.as_str() || name_unwrapped == shell_name {
            break; // We found the shell.
        }
    }

    // Try to get parent once more.
    pid = match pid_to_ppid(pid) {
        None => return None,
        Some(f) => f,
    };

    // Get the name.
    return match pid_to_name(pid) {
        None => None,
        Some(f) => Some(f),
    };
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