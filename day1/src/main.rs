use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        println!("{}", line);
    }

    println!("Hello, world!");
}
