#![allow(dead_code, unused_variables)]
use std::fs;
use std::env;

mod challenges;


fn main() {
    let args: Vec<_> = env::args().skip(1).collect();

    let err_str = "Provide a day argument like 5a";
    let command = args.get(0).expect(err_str);
    let (day, a_b) = extract_xy(command).expect(err_str);

    let file_path = format!("src/input/day{}.txt", day);
    let contents: String = fs::read_to_string(file_path).expect(&format!("Couldn't find src/input/day{}.txt", day));

    let answer_index = (2*(day - 1) + match a_b {
        'a' => 0,
        'b' => 1,
        _ => panic!("Day must be either xa or xb")
    }) as usize;

    let answer = challenges::ALL_ANSWERS[answer_index](&contents);
    println!("Day {day} part {a_b}\n{answer}")
}

fn extract_xy(element: &str) -> Option<(u32, char)> {
    let chars: Vec<char> = element.chars().collect();
    if chars.len() != 2 {
        return None;
    }

    let x_char = chars[0];
    let y_char = chars[1];

    if y_char != 'a' && y_char != 'b' {
        return None;
    }

    let x: u32 = x_char.to_digit(10)?;
    if x < 1 || x > 25 {
        return None;
    }

    Some((x, y_char))
}
