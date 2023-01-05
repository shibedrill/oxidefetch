# oxidefetch
Fully cross platform Neofetch clone written in Rust. Up to 25 times faster than Neofetch!  

![alt text](image.png "Title")  

### Why Oxidefetch?
Neofetch, being a BASH script, has a few downsides in my opinion.  
1: It's slow.  
2: It only works on platforms which have the BASH shell.  
3: It's kinda big, and I like having more compact info.  
As such, I wrote OxideFetch. How cool is that? It displays your information in a manner that's compact, and BLAZINGLY fast. 
I've measured speeds of up to 25 times faster than normal Neofetch on WSL2.

### Dependencies (Build/Install)
To build Oxidefetch, you need Cargo. You can use Cargo to automatically build and install it like so:
```cargo install --git https://github.com/shibedrill/oxidefetch```.  
From there, it *should* be in your PATH. If not, add ```~/.cargo/bin``` to your PATH.

### Dependencies (Runtime)
There's only a couple runtime dependencies for this project.  
1: ```bash``` shell installed for GPU detection on Linux/Unix/mac.  
2: ```lspci``` installed for GPU detection on Linux/Unix/mac.  
(If either of these above dependencies are absent, chances are the GPU field will simply not show up. It won't crash or anything.  
GPU detection runs on Windows without any dependencies.)  
3: Nerd fonts symbols are used in the output. Install a patched font on  your system, or patch an already installed font.

### How you can help with the project
I need to verify the output of the OS information detection libraries I'm pulling in. To do this, I need the help of people 
with varying types of systems. I've tested Arch Linux and Windows 10, but nothing else. To help, you can kindly clone this 
repo, and inside the folder, run ```cargo test -- --nocapture```, and send the resultant ```test_output.txt``` file to my 
noreply email address, or directly to me on Discord at ```Shibe Drill#9730```. This program does NOT collect information 
regarding your real name, IP, location, etc. You can look at the file it generates to be sure- it's all plaintext, babey.
