#[derive(Debug)]
struct Range (i32, i32);

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn sym_contains(&self, other: &Range) -> bool {
        self.contains(other) | other.contains(self)
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.0 <= other.1 && other.0 <= self.1
    }
}

pub fn answer(input: &str) -> String {
    let contains = input.lines().map(|line| {
        let halves: Vec<_> = line.split(',').collect();
        
        let ranges: Vec<Range> = halves
        .iter()
        .map(|s| {
            let parts: Vec<&str> = s.split('-').collect();
            let start = parts[0].parse::<i32>().unwrap();
            let end = parts[1].parse::<i32>().unwrap();
            Range(start, end)
        })
        .collect();
        
        let sym_contains = *(&ranges[0].overlaps(&ranges[1]));


        let res = if sym_contains {
            1
        } else {
            0
        };
        res
    });

    contains.sum::<i32>().to_string()
}
