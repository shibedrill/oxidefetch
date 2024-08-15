# oxidefetch 2.0.0-beta.3

Fully cross platform Neofetch clone written in Rust. Up to 25 times faster than Neofetch!  

![alt text](image.png "Example output of OxideFetch on a WSL2 Arch Linux host")  

## Why Oxidefetch

Neofetch, being a BASH script, has a few downsides in my opinion.  
1: It's slow.  
2: It only works on platforms which have the BASH shell.  
3: It's kinda big, and I like having more compact info.  
As such, I wrote OxideFetch. How cool is that? It displays your information in a manner that's compact, cross-platform, and BLAZINGLY fast. I've measured speeds of up to 25 times faster than normal Neofetch on WSL2.

### Special Thanks

The most heartfelt of thanks goes out to NamedNeon, who contributed the code to perform terminal detection, and to perform GPU detection on Darwin and Windows.  

### Features  

OxideFetch can display all of the following information:  

- Date, time, and day of week  
- Username and hostname  
- Operating system name, symbol, and matching color  
- Kernel version  
- Uptime  
- Shell  
- Terminal emulator/console  
- CPU  
- GPU  
- Memory

Also, the field-titles feature can be enabled at compile-time, which will display the name of each field in white before the information within that field. By default, it is disabled.

### Installation

Download a binary for your platform, and place it in your `$PATH`.  
Currently, only Windows (x86_64, gnu/msvc) and Linux (x86_64/aarch64, gnu/musl) have binaries available. If you want a binary for another platform, you will have to follow the instructions to build from source.

### Dependencies

#### Build/Install

To build Oxidefetch, you need Cargo. If you do not already have Cargo installed on your system, you can do so by installing Rustup- either via the [instructions on their website](https://doc.rust-lang.org/cargo/getting-started/installation.html "instructions on their website") or via your system package manager.  
You will also probably need a C/C++ compiler and a build system- most likely CMake and Visual Studio Build Tools, or GNU Make and the GNU compiler collection. You will be prompted to install these if they're not found during compilation.  
You can use Cargo, once it's installed, to automatically build and install Oxidefetch like so:
`cargo install --git https://github.com/shibedrill/oxidefetch`.  
Alternatively, you can get it from the Crates repos, using `cargo install oxidefetch`. But it might be slightly out of date.  
From there, it *should* be in your `$PATH`. If not, add `source ~/.cargo/env` to your profile, or add `~/.cargo/bin` to your `$PATH`.  

#### Runtime

The only runtime dependency for this project is a font with Nerd Fonts Symbols. If this is not used, the symbols in the output will not appear correctly.

### How you can help with the project

I need to verify the output of the OS information detection libraries I'm pulling in. To do this, I need the help of people with varying types of systems. I've tested a few, but there's some I'm unable to test. To help, you can kindly clone this repo, and inside the folder, run `cargo test -- --nocapture`, and send the resultant `test_output.txt` file to my noreply email address, or directly to me on Discord at `@shibedrill`. This program does NOT collect information regarding your real name, IP, location, hardware serial numbers, etc. You can look at the file it generates to be sure- it's all plaintext, babey. Also, consider contributing to [libpci-rs](https://github.com/namedneon/libpci-rs) to improve its functionality, efficiency, and cleanliness.  

#### Tested distributions/platforms

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

#### Semi-urgent fixes

- None so far.

#### Very near future

- Add support for user configurability for entries (whether or not an entry shows, its color, units for memory and time)  
- Add process count detection  
- Refactor logic for cleaner code  

#### Future

- Add host system name detection such as "Windows Subsystem for Linux", "IdeaPad 3", "Dell Optiplex", etc.  
- Add package count/package manager detection  
- Crosstest on more distributions to verify `sys.name()` outputs  

#### Distant future

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
**1.4.8:** Applied Clippy suggestions. Added stuff to README.  
**2.0.0-beta.0:** Switch from deprecated, platform-dependent GPU backends to [libpci-rs](https://github.com/gibsonpil/libpci-rs)  
**2.0.0-beta.1:** Updated test functionality to include package version in the log file.  
**2.0.0-beta.2:** Updated use of `libpci-rs` as its API approaches stability.  
**2.0.0-beta.3:** GPU subsystem names will display instead of long names, if available.  
**2.0.0-beta.4:** Added NixOS as a recognized distro.  

### License

This software is covered by the MIT license. See license.txt for details.
