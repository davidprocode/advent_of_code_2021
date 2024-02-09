use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn day01() {
    let file = File::open("src/day01/inputs/input.txt").expect("Cannot find the file");
    let buf = BufReader::new(file);
    let lines = buf
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    println!("lines in input: {}", lines.len());
}
