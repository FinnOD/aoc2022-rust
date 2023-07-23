use std::collections::VecDeque;


#[derive(Debug)]
struct Yard {
    stacks: Vec<Stack>
}

impl Yard {
    fn from_crates_text(text: &str) -> Yard{

        let crate_lines_full: Vec<&str> = text.split("\n").collect();
        let crate_lines = &crate_lines_full[..(crate_lines_full.len()-1)];

        let crate_rows: Vec<_> = crate_lines.into_iter().map(|line| {
            let chars_vec: Vec<char> = line
                .chars()
                .collect::<Vec<char>>();

            let crates: Vec<_> = chars_vec
                .chunks(4)
                .map(|chunk| Crate::from_char(chunk[1]))
                .collect();
            crates
        }).collect();

        let transposed_vec: Vec<Vec<_>> = (0..crate_rows[0].len())
            .map(|i| crate_rows.iter().map(|inner_vec| inner_vec[i]).collect())
            .collect();

        let a: Vec<_> = transposed_vec
            .into_iter()
            .map(|col| 
                col.into_iter()
                .fold(
                    Stack::new(),
                    |acc: Stack, crate_opt|{
                        if let Some(crate_opt) = crate_opt {
                            let mut x = acc.clone();
                            x.crates.push_back(crate_opt);
                            x
                        }else {
                            acc
                        }
                    })
            ).collect();
        
        Yard { stacks: a }
    }

    fn top_row(&self) -> Vec<Crate> {
        let crates_vec: Vec<_> = self.stacks.clone().into_iter().map(|stack| stack.crates.get(0).unwrap().clone()).collect();
        crates_vec
    }
}

#[derive(Debug, Clone)]
struct Stack {
    crates: VecDeque<Crate>
}

impl Stack {
    fn new() -> Stack {
        Stack {
            crates: VecDeque::new()
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Crate {
    id: char
}

impl Crate {

    fn from_char(c: char) -> Option<Crate> {
        if c.is_whitespace() {
            None
        } else {
            Some(Crate { id: c })
        }
    }
}

pub fn answer(input: &str) -> String {
    
    let crates_instructions: Vec<&str> =  input.split("\n\n").map(|x| x).collect();
    let crates_str = crates_instructions[0];
    let instructions_str = crates_instructions[1];

    let mut yard = Yard::from_crates_text(crates_str);

    for instruction in instructions_str.lines(){
        let ins: Vec<usize> = instruction.split(" ").skip(1).step_by(2).take(3).map(|x| x.parse().unwrap()).collect();
        
        for _ in 0..ins[0]{
            let craned = yard.stacks[ins[1]-1].crates.pop_front();
            if let Some(craned) = craned {
                yard.stacks[ins[2]-1].crates.push_front(craned);
            }
        }
    }

    let out_str: String = yard.top_row().into_iter().map(|x| x.id).collect();
    out_str

}