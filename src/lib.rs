use std::io::{self, BufRead};

use std::fs;

pub fn load_file(file_path: &str) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let full_file_string = fs::read_to_string(file_path)
        .expect("Error while reading file");
    for line in full_file_string.split("\n"){
        lines.push(line.to_string());
    }
    return lines;
}

#[allow(dead_code)]
pub fn load_from_stdin() -> Vec<String> {
    let stdin = io::stdin();
    let mut lines = Vec::new();
    for line in stdin.lock().lines() {
        lines.push(line.expect("Error while unwrapping a line"));
    }
    return lines;
}
