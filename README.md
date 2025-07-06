# sho
Show CLI is a grep alternative, entirely written in Rust. It all started as a rainy day coding, to learn more about the language Rust and its CLI libraries capabilities.
## Features in vision to implement in future
- Highlighting the searched word in the sentence.
- pdf compatibility.
- Advanced pattern matching for paths, url, etc.
## How to setup
1. Install Rust-lang and Cargo in your machine. Visit https://rust-lang.org/installation for more details.
```
git clone https://github.com/jithin-sunildas.com/sho.git
cd shoho
cargo build --release
cd 
nano .zshrc // or .bashrc
alias sho='/home/your-user/route_to_dir/sho/target/debug/sho```
