use std::{collections::HashMap, iter::zip};

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let priorities =
        zip(('a'..='z').chain('A'..='Z'), 1..=52).fold(HashMap::new(), |mut prev, (k, v)| {
            prev.insert(k, v);
            prev
        });

    let answer_1 = INPUT
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .fold(0, |acc, (a, b)| {
            for c in a.chars() {
                if b.contains(c) {
                    return acc + (*priorities.get(&c).unwrap() as usize);
                }
            }
            unreachable!();
        });

    println!("part 1: {answer_1}");

    let answer_2: usize =
        INPUT
            .lines()
            .collect::<Vec<&str>>()
            .chunks_exact(3)
            .fold(0, |acc, elves| {
                for ch in elves[0].chars() {
                    if elves[1].contains(ch) && elves[2].contains(ch) {
                        return acc + (priorities[&ch] as usize);
                    }
                }
                unreachable!();
            });
    println!("part 2: {answer_2}");
}
