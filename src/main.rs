use std::fs;
use std::path::Path;

extern crate colored;
use colored::*;

fn get_end(path: &str) -> (&str) {
    match path.split("/").last() {
        Some(st) => st,
        None => "ERROR",
    }
}

fn show(path: &str, prepend: &str, level: u32) {
    let is_file = Path::new(&path).is_file();

    if is_file {
        println!("{} {}", prepend, get_end(path).green());
    } else {
        println!("{} {}", prepend, get_end(path).yellow());
        let new_prepend = format!("  {}", prepend);
        let next_level = level + 1;
        handle_path(path, &new_prepend, next_level)
    }
}

fn handle_path(base_path: &str, prepend: &str, level: u32) {
    if level > 2 {
        println!("  {}{}...", prepend, get_end(base_path).yellow());
        return;
    }

    let paths = fs::read_dir(base_path).unwrap();

    for path in paths {
        let path_str = path.unwrap().path().to_string_lossy().to_string();
        show(&path_str, &prepend, level);
    }
}

fn main() {
    let base_path = "./";
    let initial_prepend = "";
    let level = 0;

    handle_path(&base_path, &initial_prepend, level);
}
