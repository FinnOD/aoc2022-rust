
pub fn answer(input: &str) -> String {

    let parsed: Vec<Vec<i32>> = input.trim().split("\n\n").map(|group| group.split("\n").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect();
    let sums: Vec<i32> = parsed.into_iter().map(|x| x.into_iter().sum::<i32>()).collect();
    let max =  sums.into_iter().max().unwrap();
    
    max.to_string()
}