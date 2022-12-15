mod game;
mod shape;

use std::time::Instant;

use game::{Outcome, Play};
use shape::Shape;

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let now = Instant::now();
    let (answer_1, answer_2) = parse(INPUT)
        .map(|(a, b)| {
            (
                (a.parse::<Shape>().unwrap(), b.parse::<Shape>().unwrap()),
                (a.parse::<Shape>().unwrap(), b.parse::<Outcome>().unwrap()),
            )
        })
        .fold((0, 0), |prev, (a, b)| {
            (prev.0 + a.play(), prev.1 + b.play())
        });

    let elapsed = now.elapsed().as_micros();
    println!("done in {elapsed} micro seconds");

    println!("part 1: {answer_1}");
    println!("part 2: {answer_2}");
}

fn parse(input: &str) -> impl Iterator<Item = (&str, &str)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split_once(" ").expect("line should be valid"))
}
