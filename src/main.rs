#![allow(dead_code)]

use std::fs;
mod challenges;


fn main() {
    let file_path = "src/input/day4.txt";

    let contents: String = fs::read_to_string(file_path).unwrap();
    let answer = challenges::day4a::answer(&contents);
    println!("{answer}")
}
