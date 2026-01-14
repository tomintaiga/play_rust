use std::fs::read_to_string;

/// read_lines will read all lines from text file and return them as `Vec<String>`
fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }

    result
}

/// Game entry point
fn main() {
    // File name for a game level
    let file_name = "level.txt";

    let level = read_lines(file_name);

    for cur in level {
        println!("{cur}");
    }
}
