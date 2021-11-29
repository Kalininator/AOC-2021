use std::fs;
use std::io::{BufRead, BufReader};

pub fn read_file(path: &str) -> Vec<String> {
    let file = fs::File::open(path).expect("Failed to read file");
    let br = BufReader::new(file);

    br.lines().map(|l| l.expect("Failed to get line")).collect()
}
