mod actions;
mod menu;

use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let result = if args.is_empty() {
        menu::run_general_menu()
    } else {
        let path = PathBuf::from(&args[0]);
        if !path.exists() {
            eprintln!("rclick: '{}' does not exist", args[0]);
            std::process::exit(1);
        }
        menu::run_file_menu(&path)
    };

    if let Err(e) = result {
        eprintln!("rclick {}", e);
        std::process::exit(1);
    }
}
