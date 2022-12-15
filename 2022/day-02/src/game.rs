use std::{cmp::Ordering, str::FromStr};

use crate::shape::Shape;

pub type Game = (Shape, Shape);

pub type RiggedGame = (Shape, Outcome);

#[derive(Debug, PartialEq)]
pub enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    pub fn score(&self) -> usize {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

impl FromStr for Outcome {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(format!("unknown outcome character: {s}")),
        }
    }
}

pub trait Play {
    fn play(&self) -> usize;
}

impl Play for Game {
    fn play(&self) -> usize {
        let outcome = match self.0.cmp(&self.1) {
            Ordering::Greater => Outcome::Lose,
            Ordering::Equal => Outcome::Draw,
            Ordering::Less => Outcome::Win,
        };

        outcome.score() + self.1.score()
    }
}

impl Play for RiggedGame {
    fn play(&self) -> usize {
        let choice = match self.1 {
            Outcome::Lose => match self.0 {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
            Outcome::Draw => self.0.clone(),
            Outcome::Win => match self.0 {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
        };

        choice.score() + self.1.score()
    }
}

#[cfg(test)]
mod test {
    use super::{Outcome, Play, Shape};

    #[test]
    fn play_game() {
        assert_eq!(
            (Shape::Rock, Shape::Scissors).play(),
            Outcome::Lose.score() + Shape::Scissors.score()
        );
        assert_eq!(
            (Shape::Rock, Shape::Rock).play(),
            Outcome::Draw.score() + Shape::Rock.score()
        );
        assert_eq!(
            (Shape::Rock, Shape::Paper).play(),
            Outcome::Win.score() + Shape::Paper.score()
        );

        assert_eq!(
            (Shape::Paper, Shape::Rock).play(),
            Outcome::Lose.score() + Shape::Rock.score()
        );
        assert_eq!(
            (Shape::Paper, Shape::Paper).play(),
            Outcome::Draw.score() + Shape::Paper.score()
        );
        assert_eq!(
            (Shape::Paper, Shape::Scissors).play(),
            Outcome::Win.score() + Shape::Scissors.score()
        );

        assert_eq!(
            (Shape::Scissors, Shape::Paper).play(),
            Outcome::Lose.score() + Shape::Paper.score()
        );
        assert_eq!(
            (Shape::Scissors, Shape::Scissors).play(),
            Outcome::Draw.score() + Shape::Scissors.score()
        );
        assert_eq!(
            (Shape::Scissors, Shape::Rock).play(),
            Outcome::Win.score() + Shape::Rock.score()
        );
    }

    #[test]
    fn play_rigged_game() {
        assert_eq!(
            (Shape::Rock, Outcome::Lose).play(),
            Outcome::Lose.score() + Shape::Scissors.score()
        );
        assert_eq!(
            (Shape::Rock, Outcome::Draw).play(),
            Outcome::Draw.score() + Shape::Rock.score()
        );
        assert_eq!(
            (Shape::Rock, Outcome::Win).play(),
            Outcome::Win.score() + Shape::Paper.score()
        );

        assert_eq!(
            (Shape::Paper, Outcome::Lose).play(),
            Outcome::Lose.score() + Shape::Rock.score()
        );
        assert_eq!(
            (Shape::Paper, Outcome::Draw).play(),
            Outcome::Draw.score() + Shape::Paper.score()
        );
        assert_eq!(
            (Shape::Paper, Outcome::Win).play(),
            Outcome::Win.score() + Shape::Scissors.score()
        );

        assert_eq!(
            (Shape::Scissors, Outcome::Lose).play(),
            Outcome::Lose.score() + Shape::Paper.score()
        );
        assert_eq!(
            (Shape::Scissors, Outcome::Draw).play(),
            Outcome::Draw.score() + Shape::Scissors.score()
        );
        assert_eq!(
            (Shape::Scissors, Outcome::Win).play(),
            Outcome::Win.score() + Shape::Rock.score()
        );
    }
}
