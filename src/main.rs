use colored::*;
use sys_info::*;
use users::*;
use chrono::*;

fn get_os() -> OsInfo {
    let current_info: OsInfo = match os_type() {
        Ok(os_type) => match os_type.as_ref() {
            "Windows" => OsInfo {
                name: String::from("Windows"),
                icon: '',
                term_color: String::from("bright cyan"),
            },
            "Darwin" => OsInfo {
                name: String::from("Apple Darwin"),
                icon: '',
                term_color: String::from("bright white"),
            },
            "Linux" => get_linux_info(),
            _ => OsInfo {
                name: String::from("Unknown"),
                icon: '',
                term_color: String::from("bright white"),
            },
        },
        _ => OsInfo {
            name: String::from("Unknown"),
            icon: '',
            term_color: String::from("bright white"),
        },
    };

    current_info
}

fn get_linux_info() -> OsInfo {
    match linux_os_release() {
        Ok(release_info) => match release_info.pretty_name.unwrap().as_ref() {
            "Arch Linux" => OsInfo {
                name: String::from("Arch Linux"),
                icon: '',
                term_color: String::from("bright cyan"),
            },
            "Linux Debian" => OsInfo {
                name: String::from("Linux Debian"),
                icon: '',
                term_color: String::from("red"),
            },
            "Fedora Linux" => OsInfo {
                name: String::from("Fedora Linux"),
                icon: '',
                term_color: String::from("bright blue"),
            },
            _ => OsInfo {
                name: String::from("Unknown Linux"),
                icon: '',
                term_color: String::from("bright white"),
            },
        },
        _ => OsInfo {
            name: String::from("Unknown Linux"),
            icon: '',
            term_color: String::from("bright white"),
        },
    }
}

struct OsInfo {
    name: String,
    icon: char,
    term_color: String,
}

fn main() {

    println!();

    let date = chrono::Utc::now();
    let date_string = format!("{}, {}-{}-{} at {}:{}", date.weekday(), date.year(), date.month(), date.day(), date.hour(), date.minute());
    println!("{} {}", format!("Date:").bright_white(), format!(" {}", date_string).bright_yellow());

    let current_info = get_os();
    let info_string = format!("{} {}", current_info.icon, current_info.name).color(current_info.term_color).bold();

    if hostname().is_ok() & get_current_username().is_some() {
        let username = format!("{:#?}", get_current_username().unwrap());
        println!("{} {}", String::from("Host:").bright_white(), format!(" {}@{}", &username[1..(username.len() - 1)], hostname().unwrap()).bold())
    }

    println!("{} {}", String::from("OS:").bright_white(), info_string);
    if proc_total().is_ok() {
        println!("{} {}", String::from("Procs:").bright_white(), format!(" {}", proc_total().unwrap()).magenta().bold());
    }

    if cpu_num().is_ok() & cpu_speed().is_ok() {
        println!("{} {}", String::from("CPUs:").bright_white(), format!(" {} @ {}mhz", cpu_num().unwrap(), cpu_speed().unwrap()).bright_black().bold())
    }

}
