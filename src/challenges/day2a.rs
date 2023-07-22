#[derive(Debug, Copy, Clone, PartialEq)]
enum Hand {
    Rock, Paper, Scissors
}

#[derive(Debug)]
enum HandParsingError {
    InvalidHand,
}

#[derive(Debug)]
struct Round {
    op_hand: Hand,
    player_hand: Hand,
}

impl Round {

    fn from_str(inp: &str) -> Result<Round, HandParsingError>{
        let split = inp.split_at(1);
        
        let op_hand = match split.0.trim() {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            _ => return Err(HandParsingError::InvalidHand),
        };
        let player_hand = match split.1.trim() {
            "X" => Hand::Rock,
            "Y" => Hand::Paper,
            "Z" => Hand::Scissors,
            _ => return Err(HandParsingError::InvalidHand),
        };

        Ok(
            Round {
                op_hand: op_hand,
                player_hand: player_hand
            }
        ) 
    }

    fn calculate(&self) -> i32{
        // dbg!((self.player_hand, self.op_hand));
        let player_value = match self.player_hand {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        };

        let round_value = match (&self.player_hand, &self.op_hand) {
            (player, op) if player == op => 3,
            (Hand::Rock, Hand::Scissors) | (Hand::Scissors, Hand::Paper) | (Hand::Paper, Hand::Rock) => 6,
            _ => 0
        };
        player_value + round_value
    }
}

pub fn answer(input: &str) -> String {
    let round_scores =  input.trim().split("\n").map(|x| Round::from_str(x).unwrap().calculate()).collect::<Vec<i32>>();
    // dbg!(&a);
    let sum = round_scores.into_iter().sum::<i32>();
    sum.to_string()
}