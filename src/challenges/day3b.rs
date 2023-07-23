use std::collections::HashSet;

pub fn answer(input: &str) -> String {
    let lines_vec: Vec<&str> = input.lines().collect();

    let sum: i32 = lines_vec.chunks(3).map(|x| {
        let sets = x
            .into_iter()
            .map(|z| -> HashSet<char> { HashSet::from_iter(z.chars()) });

        let intersection_iter = sets.into_iter().fold(None, |acc, set| {
            match acc {
                None => Some(set.clone()),
                Some(intersection) => Some(intersection.intersection(&set).cloned().collect()),
            }
        }).unwrap_or_else(|| HashSet::new());

        let single_char = intersection_iter.iter().next().unwrap();
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

    sum.to_string()
}
