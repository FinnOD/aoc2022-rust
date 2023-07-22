use std::collections::HashSet;

pub fn answer(input: &str) -> String {

    let sum: i32 = input
        .lines()
        .map(|x| {
            let split_tup = x.split_at(x.len() / 2);
            let sets: (HashSet<char>, HashSet<char>) = (
                HashSet::from_iter(split_tup.0.chars()),
                HashSet::from_iter(split_tup.1.chars()),
            );
            let intersection = sets.0.intersection(&sets.1).cloned().collect::<Vec<_>>();
            let single_char = intersection.get(0).unwrap();
            single_char.to_owned()
        })
        .map(|x| {
            let  offset: i32 = match x.is_lowercase() {
                true => -32,
                false => 26,
            };
            let ascii = (x as u32) as i32;
            let a_offset: i32 = ('A' as u32) as i32;
            let adjusted = (ascii - a_offset) + 1 + offset;
            adjusted
        })
        .sum();

    // dbg!(sum);
    sum.to_string()
}
