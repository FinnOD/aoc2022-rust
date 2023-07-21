
pub fn answer(input: &str) -> String {

    let parsed: Vec<Vec<i32>> = input.trim().split("\n\n").map(|group| group.split("\n").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect();
    let mut sums: Vec<i32> = parsed.into_iter().map(|x| x.into_iter().sum::<i32>()).collect();
    sums.sort();
    let last_three_sum: i32 = sums.into_iter().rev().take(3).sum();
   
    last_three_sum.to_string()
}