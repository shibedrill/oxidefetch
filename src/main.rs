use colored::*;
use sysinfo::*;
use whoami::*;
use chrono::*;

fn main() {
    let sys_info = InformationStruct::new();
    println!("{}", sys_info.os_name);
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
            icon: '?',
        }
    }
}