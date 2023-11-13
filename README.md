# oxidefetch 1.4.7
Fully cross platform Neofetch clone written in Rust. Up to 25 times faster than Neofetch!  

![alt text](image.png "Example output of OxideFetch on a WSL2 Arch Linux host")  

### Why Oxidefetch?
Neofetch, being a BASH script, has a few downsides in my opinion.  
1: It's slow.  
2: It only works on platforms which have the BASH shell.  
3: It's kinda big, and I like having more compact info.  
As such, I wrote OxideFetch. How cool is that? It displays your information in a manner that's compact, cross-platform, and BLAZINGLY fast. I've measured speeds of up to 25 times faster than normal Neofetch on WSL2.

### Special Thanks
The most heartfelt of thanks goes out to NamedNeon, who contributed the code to perform terminal detection.  

### Features  
OxideFetch can display all of the following information:  
- Date, time, and day of week  
- Username and hostname  
- Operating system name, symbol, and matching color  
- Kernel version  
- Uptime  
- Shell  
- Terminal  
- CPU  
- GPU  
- Memory  
- Terminal emulator  

### Installation  
Download a binary for your platform, and place it in your PATH.  
Currently, only Windows (x86) and Linux (x86) have binaries available. If you want a binary for another platform, you will have to follow the instructions to build from source.

### Dependencies 
#### Build/Install
To build Oxidefetch, you need Cargo. You can use Cargo to automatically build and install it like so:
`cargo install --git https://github.com/shibedrill/oxidefetch`. (Alternatively, you can get it from the Crates repos, using `cargo install oxidefetch`. But it might be slightly out of date.) From there, it *should* be in your `$PATH`. If not, add `source ~/.cargo/env` to your profile, or add `~/.cargo/bin` to your `$PATH`. If you do not already have Cargo installed on your system, you can do so by installing Rustup- either via the [instructions on their website](https://doc.rust-lang.org/cargo/getting-started/installation.html "instructions on their website") or via your system package manager. You will also probably need a C compiler and a build system- most likely CMake or GNU Make. You will be prompted to install these if they're not found during compilation.
#### Runtime
There's only a couple runtime dependencies for this project.  
1: `sh` shell installed for GPU detection on \*nix systems.  
2: `lspci` installed for GPU detection on \*nix systems.  
(If either of these above dependencies are absent, chances are the GPU field will simply not show up. It won't crash or anything.  
GPU detection runs on Windows without any dependencies.)  
3: Nerd fonts symbols are used in the output. Install a patched font on  your system, or patch an already installed font.

### How you can help with the project
I need to verify the output of the OS information detection libraries I'm pulling in. To do this, I need the help of people with varying types of systems. I've tested a few, but there's some I'm unable to test. To help, you can kindly clone this repo, and inside the folder, run `cargo test -- --nocapture`, and send the resultant `test_output.txt` file to my noreply email address, or directly to me on Discord at `@shibedrill`. This program does NOT collect information regarding your real name, IP, location, hardware serial numbers, etc. You can look at the file it generates to be sure- it's all plaintext, babey.  
#### Tested distributions/platforms:
- Alma Linux
- Alpine Linux
- Arch Linux
- CentOS
- Debian GNU/Linux
- Fedora
- Gentoo
- Kali GNU/Linux
- Linux Mint
- openSUSE Leap
- openSUSE Tumbleweed
- Ubuntu
- Windows

### Bugs, Quirks, Unintended Behavior, And Other Shenanigans   
No weird quirks to report at this time.  

### To Do & Roadmap   
#### Semi-urgent fixes:   
- None so far.
#### Very near future:   
- Add support for user configurability for entries (whether or not an entry shows, its color, units for memory and time)   
- Add process count detection  
- Refactor logic for cleaner code  
- Edit \*nix GPU detection to include GPUs that do not include `VGA Compatible Controller` in the name  
#### Future:  
- Add host system name detection such as "Windows Subsystem for Linux", "IdeaPad 3", "Dell Optiplex", etc.  
- Add package count/package manager detection  
- Crosstest on more distributions to verify `sys.name()` outputs  
- Refactor GPU detection logic into separate crate, remove dependencies on `sh` and `lspci`, and put any platform-specific code in separate files (This is slated for the 2.0.0 release. If you want to help accelerate this effort, consider contributing to [libpci-rs](https://github.com/namedneon/libpci-rs).)  
#### Distant future:
- More extensible user configuration for entry formatting  
- Separate all information-getting logic into a new Fetch crate, allowing people to make their own fetch programs using a unified cross-platform API  

### Changelog
**1.0.0:** Official full stable release   
**1.0.1:** Fixed distro name for Debian GNU/Linux. Logo & color works now.  
**1.1.0:** Refactored some poorly written typing, and added support for memory.  
**1.1.1:** Made sure that linux system detection won't fail if Linux has a capital L.  
**1.1.2:** Replaced *nix dependency on `bash` with dependency on `sh`.  
**1.2.0:** Allowed users to enable field titles as a compile-time feature. Tentative fix for GPU display issues on Linux.  
**1.2.1:** Stable fix for GPU display quirks.  
**1.2.2:** All GPUs should print in their own lines.  
**1.3.0:** Tentative fix for issue where empty GPU info line might print on Linux.  
**1.3.2:** Changed color of time output to be more visible on gray terminals.  
**1.4.0:** Added support for terminal detection, and fixed system detection on Darwin systems.  
**1.4.1:** Changed terminal color to match shell color.  
**1.4.2:** Updated colors and logos of a few distros. They will now display correctly.  
**1.4.3:** Removed newline print before information. This should be up to the user to print, using their shell profile.  
**1.4.4:** Fixed an issue where GPUs would all print on one line.  
**1.4.5:** Minor changes to system color detection. Removed all warnings.  
**1.4.6:** Cargo formatting applied to all files. Mild string reformatting in print statements.  
**1.4.7:** Removed several `unwrap()` calls. Changed debug output to serialize to RON.  

### License
This software is covered by the MIT license. See license.txt for details.
