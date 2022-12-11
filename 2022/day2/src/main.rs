use std::fs;
use std::error::Error;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Move {
    fn score(&self) -> u64 {
        match *self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
    
    fn parse(move_str: &str) -> Result<Self, String> {
        match move_str {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            _ => Err(format!("Unrecognized move string: {}", move_str)),
        }
    }
    
    fn wins_against(&self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }
    
    fn loses_against(&self) -> Move {
        self.wins_against().wins_against()
    }
    
    fn outcome(&self, other: &Self) -> Outcome {
        if self.wins_against() == *other {
            Outcome::Win
        } else if self.loses_against() == *other {
            Outcome::Lose
        } else {
            Outcome::Draw
        }
    }
}

impl Outcome {
    fn score(&self) -> u64 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
    
    fn parse(move_str: &str) -> Result<Self, String>{
        match move_str {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(format!("Unrecognized outcome string: {}", move_str))
        }
    }
    
    fn move_for(&self, player: &Move) -> Move {
        match self {
            Outcome::Win => player.loses_against(),
            Outcome::Lose => player.wins_against(),
            Outcome::Draw => *player,
        }
    }
}

fn get_total_score() -> Result<u64, Box<dyn Error>> {
    Ok(fs::read_to_string("input.txt").unwrap().lines().map(|line| {
        line.split(" ").collect::<Vec<&str>>()
    }).map(|move_string_pair| {
        if move_string_pair.len() != 2 {
            Err(format!("Line does not contain two strings: {:?}", move_string_pair))
        } else {
            Ok((Move::parse(move_string_pair[0]), Outcome::parse(move_string_pair[1])))
        }
    }).map(|pair| {
        match pair {
            Ok(move_pair) => {
                let opponent_move = move_pair.0.unwrap();
                let outcome = move_pair.1.unwrap();
                let player_move = outcome.move_for(&opponent_move);
                player_move.outcome(&opponent_move).score() + player_move.score()
            },
            Err(_) => 0,
        }
    }).sum::<u64>())
}

fn main() {
    println!("{}", get_total_score().unwrap());
}
