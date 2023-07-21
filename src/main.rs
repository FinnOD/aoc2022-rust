#![allow(dead_code)]

use std::fs;
mod challenges;


fn main() {
    let file_path = "src/input/day2.txt";

    let contents: String = fs::read_to_string(file_path).unwrap();
    let answer = challenges::day2b::answer(&contents);
    println!("{answer}")
}
