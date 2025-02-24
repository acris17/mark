use std::env;

use toml::Table;
use xdg::BaseDirectories;

use crate::sys;

pub fn open(marks: &Table, name: &str) {
    let Some(path) = marks.get(name) else {
        eprintln!("Error: Could not find the mark for name: {name}");
        return;
    };
    let process = format!("open {path}");

    sys::shell(&process);
}
pub fn edit(base_dirs: &BaseDirectories) {
    let Some(mark_file) = base_dirs.find_data_file("marks.toml") else {
        eprintln!("Error: Could not find the mark file");
        return;
    };
    let Ok(editor) = env::var("EDITOR") else {
        eprintln!("Error: EDITOR environment variable is not set");
        return;
    };

    let process = format!("{editor} {}", mark_file.display());

    sys::shell(&process);
}
pub fn get(marks: &Table, name: &str) {
    let Some(value) = marks.get(name) else {
        return;
    };
    let Some(path) = value.as_str() else {
        return;
    };

    println!("{path}");
}
