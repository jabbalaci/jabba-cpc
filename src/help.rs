use crate::config as cfg;

pub fn print_help() {
    let text = format!("\
cpc (copy path to clipboard) v{ver}

- copies the path of the current working directory to the clipboard
- if a parameter is given, it's also added to the path

Usage: cpc [option] [parameter]
where option can be:
    -h or --help            get this help

see https://github.com/jabbalaci/jabba-cpc for more info", ver=cfg::VERSION);

    println!("{}", text);
}
