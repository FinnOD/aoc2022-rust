#![allow(dead_code)]

use std::fs;
mod challenges;


fn main() {
    let file_path = "src/input/day3.txt";

    let contents: String = fs::read_to_string(file_path).unwrap();
    let answer = challenges::day3a::answer(&contents);
    println!("{answer}")
}
