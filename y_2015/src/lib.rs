use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse_input(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let mut input: Vec<String> = Vec::new();
    for line in reader.lines() {
        input.push(line.unwrap());
    }
    input
}
