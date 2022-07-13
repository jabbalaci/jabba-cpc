mod clipboard;
mod config;
mod help;

use clipboard as cb;
use std::env;
use std::process;

// enable it if you want some feedback
const DEBUG: bool = false;

fn main() {
    let arg = env::args().nth(1).unwrap_or_default();

    if ["-h", "--help"].contains(&arg.as_str()) {
        help::print_help();
        process::exit(0);
    }
    // else
    let mut path = env::current_dir().unwrap();
    if !arg.is_empty() {
        path.push(arg);
    }
    let path = path.to_string_lossy().to_string();
    if DEBUG {
        println!("# {:?}", path);
    }
    cb::set_text(&path);
}
