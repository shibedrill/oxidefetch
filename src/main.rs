/*

Copyright 2023 shibedrill & Namedneon

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the “Software”), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
of the Software, and to permit persons to whom the Software is furnished to do
so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

*/

mod terminal;

use crate::terminal::get_terminal;
use byte_unit::*;
use chrono::*;
use colored::*;
use libpci_rs::{pci::*, ids::*};
use std::env;
use sysinfo::*;

#[cfg(test)]
use serde::Serialize;

fn main() {
    // Generate system info struct
    let sys_info = Information::new();

    // Format the date and time
    let datetime_formatted = format!(
        "{}, {}",
        Utc::now().weekday(),
        Utc::now().format("%H:%M %Y-%m-%d")
    );

    // TODO: Add support to change what items print, as well as their colors.
    // This should be done via some sort of user accessible, persistent config,
    // and preferably can be modified via env vars.
    println!("{}", String::from(">>> OxideFetch  <<<").red());
    color_print("Date:\t", '󰃰', &Some(datetime_formatted), "bright yellow");
    color_print(
        "Host:\t",
        '',
        &Some(format!("{}@{}", sys_info.username, sys_info.hostname)),
        "purple",
    );
    color_print("OS:\t", sys_info.icon, &sys_info.os_name, &sys_info.color);
    color_print("Ver:\t", '', &sys_info.os_ver, "bright red");
    color_print("Kernel:\t", '', &sys_info.kernel_ver, "bright blue");
    color_print("Uptime:\t", '', &Some(sys_info.uptime), "bright gray");
    color_print("Shell:\t", '', &sys_info.shell, "bright magenta");
    color_print("Terminal:\t", '', &sys_info.terminal, "magenta");
    color_print("CPU:\t", '', &Some(sys_info.cpu), "green");

    if let Some(gpuvec) = sys_info.gpu {
        for gpu in gpuvec {
            color_print("GPU:\t", '󰍹', &Some(gpu), "bright green")
        }
    }

    color_print("Memory:\t", '󰍛', &Some(sys_info.memory), "bright blue");
}

#[allow(unused_variables)] // The field title var is sometimes unused due to compile time features
fn color_print(field_title: &str, icon: char, field: &Option<String>, color: &str) {
    // If the field is missing, it won't print.
    if let Some(fieldvalue) = field {
        #[cfg(feature = "field-titles")]
        print!("{} ", field_title.bright_white());
        println!("{}", format!("{} {}", icon, fieldvalue).color(color));
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(Serialize))]
struct Information {
    // Only fields whose setters can fail, are given Option<String> types.
    // Unsure if I should coerce these fields into an Option<String> *here*, or
    // within the args of color_print, since that function only accepts args of
    // type Option<String>.
    username: String,
    hostname: String,
    os_name: Option<String>,
    os_ver: Option<String>,
    kernel_ver: Option<String>,
    uptime: String,
    shell: Option<String>,
    terminal: Option<String>,
    cpu: String,
    gpu: Option<Vec<String>>,
    memory: String,
    icon: char,
    color: String,
}

impl Information {
    fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let mut os_name = sys.name();

        // Get full OS name for Darwin-based systems (i.e. macOS, iOS).
        if os_name == Some("Darwin".to_string()) {
            let long_os = sys.long_os_version();
            if long_os.is_some() {
                // Isolate system type from version information.
                let long_os_split =
                    long_os.unwrap().split_whitespace().collect::<Vec<&str>>()[0].to_string();

                os_name = Some(long_os_split);
            }
        }

        Self {
            username: whoami::username(),
            hostname: whoami::hostname(),
            os_name: os_name.clone(),
            os_ver: sys.os_version(),
            kernel_ver: sys.kernel_version(),
            uptime: compound_duration::format_dhms(sys.uptime()),

            // Tracks the SHELL env var and trims the last item from the resultant fs path.
            shell: {
                let var = env::var("SHELL");
                if let Ok(var_ok) = var {
                    Some(var_ok.split('/').last().unwrap().to_string())
                } else {
                    None
                }
            },

            terminal: get_terminal(),
            cpu: String::from(sys.cpus()[0].brand()),

            gpu: {
                if let Ok(pci_list) = get_pci_list() {
                    let mut gpu_name_vec: Vec<String> = vec![];
                    for device in pci_list {
                        if device.class == 3 {
                            let vendor_entry = lookup_vendor(device.vendor_id).unwrap();
                            let device_entry = vendor_entry.device(device.device_id).unwrap();
                            gpu_name_vec.push(format!(
                                "{} {} {}",
                                vendor_entry.name(),
                                device_entry.name(),
                                {
                                    if device.revision_id != 0 {
                                        format!(" (rev {:02x})", device.revision_id)
                                    } else {
                                        "".to_string()
                                    }
                                }
                            ));
                        }
                    }
                    match gpu_name_vec.len() {
                        0 => None,
                        1.. => Some(gpu_name_vec),
                    }
                } else {
                    None
                }
            },

            memory: format!(
                "{}/{}",
                Byte::from(sys.used_memory()).get_appropriate_unit(true),
                Byte::from(sys.total_memory()).get_appropriate_unit(true)
            ),

            icon: match os_name
                .unwrap_or(String::from("Unknown System"))
                .as_ref()
            // Getting the icon for the distro.
            // I have NO clue if these are the strings the
            // sys.name() function will return.
            // TODO: Validate sys.name() outputs.
            {
                "AlmaLinux" => '',
                "Alpine Linux" => '',
                "Arch Linux" => '',
                "CentOS Linux" | "CentOS Stream" => '',
                "Debian GNU/Linux" => '',
                "ElementaryOS" => '',
                "EndeavourOS" => '',
                "Fedora Linux" => '',
                "FreeBSD" => '',
                "Gentoo" => '',
                "Kali GNU/Linux" => '',
                "Linux Mint" => '',
                "Manjaro Linux" => '',
                "openSUSE Tumbleweed" | "openSUSE Leap" => '',
                "PopOS" => '',
                "Ubuntu" => '',
                "Windows" => '',
                "Android" => '',
                "iOS" => '',
                "MacOS" => '',
                "Unknown System" => '?',
                _ => {
                    if sys
                    .name()
                    .unwrap_or(String::from("Unknown System"))
                    .to_ascii_lowercase()
                    .contains("linux") {
                        // If we don't know what it is exactly, but we know it's Linux,
                        // just toss in good ol' Tux.
                        ''
                    } else {
                        // If we have ZERO clue what it is, just use a question mark.
                        '?'
                    }
                }
            },

            color: String::from(
                match sys
                .name()
                .unwrap_or(String::from("Unknown System"))
                .as_ref()
            // Again, I don't know whether this is what the strings will look like.
            // Feel free to fix if it's broken on your system.
            {
                "Debian GNU/Linux" => "bright red",
                "FreeBSD" => "red",
                "Ubuntu" => "orange",
                "Arch Linux" | "Windows" | "PopOS" => "bright cyan",
                "Fedora Linux" | "Kali GNU/Linux" | "Alpine Linux" => "bright blue",
                "openSUSE Tumbleweed" | "openSUSE Leap" | "Linux Mint" | "Android" => "bright green",
                "EndeavourOS" | "Gentoo" | "CentOS Linux" | "CentOS Stream" => "purple",
                "iOS" | "macOS" | "ElementaryOS" => "bright white",
                "AlmaLinux" => "yellow",

                _ => "bright white",
            },
            ),
        }
    }
}

#[cfg(test)]
mod test {

    use crate::Information;
    use std::fs;

    // Self explanatory.
    #[test]
    pub fn log_gathered_data() {
        let sys_info = Information::new();
        //let data_string = format!("{:#?}", sys_info);
        let data_string = format!(
            "Version: {}\nBegin structure dump:\n{}",
            env!("CARGO_PKG_VERSION"),
            ron::ser::to_string_pretty(&sys_info, ron::ser::PrettyConfig::default())
                .expect("Failed to serialize data structure. Aborting...")
        );
        let result = fs::write("./test_output.ron", data_string);

        if result.is_ok() {
            println!(
                "
HEY THERE! A logging file was generated by this test. \
It's located in this folder, and called 'test_output.ron'. \
SEND THIS FILE to the maintainer of the project!
            "
            );
        } else {
            println!(
                "
Woops. A file wasn't able to be generated. \
The program or user might not have permission to make files in this directory.
            "
            );
        };

        assert!(result.is_ok());
    }
}
