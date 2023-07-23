pub mod day1a;
pub mod day1b;
pub mod day2a;
pub mod day2b;
pub mod day3a;
pub mod day3b;
pub mod day4a;
pub mod day4b;
pub mod day5a;
// pub mod day5b;
// pub mod day6a;
// pub mod day6b;
// pub mod day7a;
// pub mod day7b;

pub const ALL_ANSWERS: [fn(&str) -> String; 9] = [
    day1a::answer,
    day1b::answer,
    day2a::answer,
    day2b::answer,
    day3a::answer,
    day3b::answer,
    day4a::answer,
    day4b::answer,
    day5a::answer,
    // day5::answer,
];