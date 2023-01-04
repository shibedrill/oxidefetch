
use std::fmt;
use colored::*;
use sysinfo::*;
use whoami::*;
use chrono::*;

fn main() {
    let sys_info = InformationStruct::new();
    let datetime_formatted = format!("{}, {}", Utc::now().weekday(), Utc::now().format("%H:%M %Y-%m-%d"));

    color_print("Date", '', &datetime_formatted, "bright yellow");
    color_print("Host", '', &format!("{}@{}", sys_info.username, sys_info.hostname), "purple");
    color_print("OS", sys_info.icon, &sys_info.os_name, &sys_info.color);
    color_print("Version", '', &sys_info.os_ver, "red");
    color_print("Kernel", '', &sys_info.kernel_ver, "bright blue");
    color_print("Uptime", 'ﮫ', &format!("{}s", sys_info.uptime), "bright black");
    color_print("CPU", '', &sys_info.cpu, "green");
    
}

fn color_print(field_title: &str, icon: char, field: &str, color: &str) {
    println!("{}: {}", field_title.bright_white(), format!("{} {}", icon, field).color(color));
}

struct InformationStruct {
    username: String,
    hostname: String,
    os_name: String,
    os_ver: String,
    kernel_ver: String,
    uptime: u64,
    shell: String,
    terminal: String,
    cpu: String,
    gpu: String,
    memory: String,
    icon: char,
    color: String,
}

impl InformationStruct {
    fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        Self{
            username: whoami::username(),
            hostname: whoami::hostname(),
            os_name: sys.name().unwrap_or(String::from("Unknown System")),
            os_ver: sys.os_version().unwrap_or(String::from("Unknown System Version")),
            kernel_ver: sys.kernel_version().unwrap_or(String::from("Unknown Kernel Version")),
            uptime: sys.uptime(),
            shell: String::from("Unknown Shell"),
            terminal: String::from("Unknown Terminal"),
            cpu: String::from(sys.cpus()[0].brand()),
            gpu: String::from("Unknown GPU"),
            memory: String::from("Unknown memory"),
            icon: match sys.name().unwrap_or(String::from("Unknown System")).as_ref() {
                "Alma Linux" => '',
                "Alpine Linux" => '',
                "Arch Linux" => '',
                "CentOS" => '',
                "Linux Debian" => '',
                "ElementaryOS" => '',
                "EndeavourOS" => '',
                "Fedora Linux" => '',
                "FreeBSD" => '',
                "Gentoo Linux" => '',
                "Kali Linux" => '',
                "Linux Mint" => '',
                "Manjaro Linux" => '',
                "OpenSUSE" => '',
                "PopOS" => '',
                "Ubuntu Linux" => '',
                "Windows" => '',
                "Android" => '',
                "iOS" => '',
                "macOS" => '',
                "Unknown System" => '?',
                _ => {
                    unreachable!()
                },
            },
            color: match sys.name().unwrap_or(String::from("Unknown System")).as_ref() {
                "Linux Debian" => String::from("bright red"),
                "FreeBSD" => String::from("red"),
                "Ubuntu Linux" => String::from("orange"),
                "Arch Linux" | "Windows" | "PopOS" => String::from("bright cyan"),
                "Fedora Linux" | "Kali Linux" => String::from("bright blue"),
                "OpenSUSE" | "Linux Mint" | "Android" => String::from("bright green"),
                "EndeavourOS" | "Gentoo linux" => String::from("purple"),
                "iOS" | "macOS" | "ElementaryOS" => String::from("bright white"),

                _ => String::from("bright white"),
            },
        }
    }

}