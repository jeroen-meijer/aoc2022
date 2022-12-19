#![allow(unused)] // FIXME

use super::{Answer, Assignment};

pub fn get_assignment() -> Assignment {
    return Assignment::new(2, 1, "Rock Paper Scissors".to_string(), Answer::None, _run);
}

#[derive(Eq, PartialEq, Hash, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn get_move_this_wins_over(&self) -> Move {
        match *self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    fn get_move_this_loses_to(&self) -> Move {
        match *self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn get_points(&self) -> u16 {
        match *self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

#[derive(Debug)]
struct MoveFromStringParsingError {
    character: &'static str,
}

impl TryFrom<&'static str> for Move {
    type Error = MoveFromStringParsingError;

    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(MoveFromStringParsingError { character: value }),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum GameResult {
    Win,
    Loss,
    Draw,
}

struct Game {
    player: Move,
    opponent: Move,
}

impl Game {
    fn get_result(&self) -> GameResult {
        if self.opponent == self.player.get_move_this_wins_over() {
            GameResult::Win
        } else if self.opponent == self.player.get_move_this_loses_to() {
            GameResult::Loss
        } else {
            GameResult::Draw
        }
    }
}

fn _run(_data: Vec<String>) -> Answer {
    Answer::None

    // let mut games: Vec<Game> = vec![];

    // for line in data {
    //     let mut s = line.split(' ');
    //     games.push(Game {
    //         opponent: s.next().unwrap().try_into().unwrap(),
    //         player: s.next().unwrap().clone().try_into().unwrap(),
    //     });
    // }

    // let mut scores: Vec<u32> = vec![];

    // for game in games {
    //     let move_points = game.player.get_points();
    //     let game_points: u16 = match game.get_result() {
    //         GameResult::Win => 6,
    //         GameResult::Loss => 3,
    //         GameResult::Draw => 0,
    //     };

    //     scores.push(u32::from(move_points + game_points))
    // }

    // let sum = scores.iter().sum::<u32>();

    // Some(sum as i32)
}

#[cfg(test)]
mod tests {
    use test_case::case;

    use super::*;

    #[case(Move::Rock => Move::Scissors; "when move is rock")]
    #[case(Move::Paper => Move::Rock; "when move is paper")]
    #[case(Move::Scissors => Move::Paper; "when move is scissors")]
    fn move_get_move_this_wins_over_works_correctly(m: Move) -> Move {
        m.get_move_this_wins_over()
    }

    #[case(Move::Rock => Move::Paper; "when move is rock")]
    #[case(Move::Paper => Move::Scissors; "when move is paper")]
    #[case(Move::Scissors => Move::Rock; "when move is scissors")]
    fn move_get_move_this_loses_to_works_correctly(m: Move) -> Move {
        m.get_move_this_loses_to()
    }

    #[case(Move::Rock => 1; "when move is rock")]
    #[case(Move::Paper => 2; "when move is paper")]
    #[case(Move::Scissors => 3; "when move is scissors")]
    fn move_get_points_works_correctly(m: Move) -> u16 {
        m.get_points()
    }

    #[case("A" => Move::Rock; "when input is A")]
    #[case("X" => Move::Rock; "when input is X")]
    #[case("B" => Move::Paper; "when input is B")]
    #[case("Y" => Move::Paper; "when input is Y")]
    #[case("C" => Move::Scissors; "when input is C")]
    #[case("Z" => Move::Scissors; "when input is Z")]
    fn move_try_from_str_trait_works_correctly(s: &'static str) -> Move {
        s.try_into().unwrap()
    }

    #[case(Move::Rock, Move::Rock => GameResult::Draw; "when both moves are rock")]
    #[case(Move::Rock, Move::Paper => GameResult::Loss; "when player plays rock and opponent plays rock")]
    #[case(Move::Rock, Move::Scissors => GameResult::Win; "when player plays rock and opponent plays scissors")]
    #[case(Move::Paper, Move::Rock => GameResult::Win; "when player plays paper and opponent plays rock")]
    #[case(Move::Paper, Move::Paper => GameResult::Draw; "when both moves are paper")]
    #[case(Move::Paper, Move::Scissors => GameResult::Loss; "when player plays paper and opponent plays scissors")]
    #[case(Move::Scissors, Move::Rock => GameResult::Loss; "when player plays scissors and opponent plays rock")]
    #[case(Move::Scissors, Move::Paper => GameResult::Win; "when player plays scissors and opponent plays paper")]
    #[case(Move::Scissors, Move::Scissors => GameResult::Draw; "when both moves are scissors")]
    fn game_get_result_works_correctly(player: Move, opponent: Move) -> GameResult {
        Game { player, opponent }.get_result()
    }
}
