/*

Copyright 2023 shibedrill

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

use byte_unit::*;
use chrono::*;
use colored::*;
use compound_duration;
use std::env;
use sysinfo::*;
use whoami;

fn main() {
    // Generate system info struct
    let sys_info = InformationStruct::new();

    // Format the date and time
    let datetime_formatted = format!(
        "{}, {}",
        Utc::now().weekday(),
        Utc::now().format("%H:%M %Y-%m-%d")
    );

    println!();

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
    color_print("CPU:\t", '', &Some(sys_info.cpu), "green");
    if let Some(gpus) = sys_info.gpu {
        for gpu in gpus {
            color_print("GPU:\t", '󰍹', &Some(gpu), "bright green")
        }
    }
    //color_print("GPU:\t", '', &sys_info.gpu, "bright green");
    color_print("Memory:\t", '󰍛', &Some(sys_info.memory), "bright blue");
}

fn color_print(field_title: &str, icon: char, field: &Option<String>, color: &str) {
    // If the field is missing, it won't print.
    if field.is_some() {
        #[cfg(feature = "field-titles")]
        print!("{} ", field_title.bright_white());
        println!(
            "{}",
            format!("{} {}", icon, field.as_ref().unwrap()).color(color)
        );
    }
}

#[derive(Debug)]
struct InformationStruct {
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
    _terminal: Option<String>,
    cpu: String,
    gpu: Option<Vec<String>>,
    memory: String,
    icon: char,
    color: String,
}

impl InformationStruct {
    fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        Self {
            username: whoami::username(),

            hostname: whoami::hostname(),

            os_name: sys.name(),

            os_ver: sys.os_version(),

            kernel_ver: sys.kernel_version(),

            uptime: compound_duration::format_dhms(sys.uptime()),

            // Tracks the SHELL env var and trims the last item from the resultant fs path.
            shell: {
                let var = env::var("SHELL");
                if var.is_ok() {
                    Some(format!("{}", var.unwrap().split('/').last().unwrap()))
                } else {
                    None
                }
            },

            _terminal: None, // TODO: Add terminal detection.

            cpu: String::from(sys.cpus()[0].brand()),

            gpu: {
                match sys
                    .name()
                    .unwrap_or(String::from("Unknown System"))
                    .as_ref()
                {
                    "Windows" => {
                        // On windows, we run "wmic path win32_VideoController get name" and
                        // the second line is our GPU name.
                        let command_output = std::process::Command::new("wmic")
                            .args(["path", "win32_VideoController", "get", "name"])
                            .output();
                        match command_output {
                            Ok(gpu_info) => {
                                let gpu_info_as_string = String::from_utf8(gpu_info.stdout);
                                Some(vec![String::from(
                                    gpu_info_as_string
                                        .unwrap() // TODO: Please figure out a way to get rid of this unwrap() call.
                                        // I feel like I did so well avoiding unwrap calls everywhere except for here.
                                        .split("\n")
                                        .collect::<Vec<&str>>()[1],
                                )])
                            }
                            Err(_) => None,
                        }
                    }
                    _ => {
                        // On *nix, hopefully, "lspci | grep VGA | awk -F 'VGA compatible controller: ' '{print $2}'" gives us our GPU name.
                        // Since pipes can't be processed as arguments, we need to do all this in a subshell under SH.
                        let command_output = std::process::Command::new("sh")
                            .args(["-c", "lspci | grep VGA | awk -F 'VGA compatible controller: ' '{print $2}'"])
                            .output();

                        // Check if running the command resulted in an error. If not, convert to a vector.
                        // TODO: Please fix this horrible logic. It needs refactoring.
                        match command_output {
                            Err(_) => None,
                            Ok(output_bytes) => {
                                match String::from_utf8(output_bytes.stdout) {
                                    Err(_) => None,
                                    Ok(output_string) => {
                                        match output_string.as_ref() {
                                            "" => None,
                                            _ => {
                                                Some(vec![
                                                    output_string
                                                    .split("\n")
                                                    .collect()
                                                ])
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },

            memory: format!(
                "{}/{}",
                Byte::from(sys.used_memory()).get_appropriate_unit(true),
                Byte::from(sys.total_memory()).get_appropriate_unit(true)
            ),

            icon: match sys
                .name()
                .unwrap_or(String::from("Unknown System"))
                .as_ref()
            // Getting the icon for the distro.
            // I have NO clue if these are the strings the
            // sys.name() function will return.
            // TODO: Validate sys.name() outputs.
            {
                "Alma Linux" => '',
                "Alpine Linux" => '',
                "Arch Linux" => '',
                "CentOS" => '',
                "Debian GNU/Linux" => '',
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

            color: match sys
                .name()
                .unwrap_or(String::from("Unknown System"))
                .as_ref()
            // Again, I don't know whether this is what the strings will look like.
            // Feel free to fix if it's broken on your system.
            {
                "Debian GNU/Linux" => String::from("bright red"),
                "FreeBSD" => String::from("red"),
                "Ubuntu Linux" => String::from("orange"),
                "Arch Linux" | "Windows" | "PopOS" => String::from("bright cyan"),
                "Fedora Linux" | "Kali Linux" | "Alpine Linux" => String::from("bright blue"),
                "OpenSUSE" | "Linux Mint" | "Android" => String::from("bright green"),
                "EndeavourOS" | "Gentoo linux" | "CentOS" => String::from("purple"),
                "iOS" | "macOS" | "ElementaryOS" => String::from("bright white"),
                "Alma Linux" => String::from("yellow"),

                _ => String::from("bright white"),
            },
        }
    }
}

#[cfg(test)]
mod test {

    use crate::InformationStruct;
    use std::fs;

    // Self explanatory.
    #[test]
    pub fn log_gathered_data() {
        let sys_info = InformationStruct::new();
        let data_string = format!("{:#?}", sys_info);
        let result = fs::write("./test_output.txt", data_string);

        if result.is_ok() {
            println!("\nHEY THERE! A logging file was generated by this test. \nIt's located in this folder, and called 'test_output.txt'. \nSEND THIS FILE to the maintainer of the project!\n");
        } else {
            println!("Woops. A file wasn't able to be generated. The program or user might not have permission to make files in this directory.");
        };

        assert!(result.is_ok());
    }
}
