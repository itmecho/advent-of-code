use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Board {
    rows: Vec<Vec<Node>>,
}

impl Board {
    pub fn mark_number(&mut self, value: usize) -> Option<(usize, usize)> {
        for (row_number, row) in self.rows.iter_mut().enumerate() {
            for (col_number, node) in row.iter_mut().enumerate() {
                if node.value == value {
                    node.marked = true;
                    return Some((row_number, col_number));
                }
            }
        }
        None
    }

    pub fn check_for_win(&self, row: usize, col: usize) -> bool {
        let complete_row = self.rows.get(row).unwrap().iter().all(|node| node.marked);
        if complete_row {
            return true;
        }
        self.rows.iter().all(|row| row.get(col).unwrap().marked)
    }

    pub fn total(&self) -> usize {
        self.rows
            .iter()
            .map(|row| {
                row.iter()
                    .filter_map(|node| if !node.marked { Some(node.value) } else { None })
                    .sum::<usize>()
            })
            .sum()
    }

    pub fn print_board(&self) {
        for row in &self.rows {
            for node in row {
                if node.marked {
                    print!("\x1b[01;32m{:>4}\x1b[00m", node.value)
                } else {
                    print!("{:>4}", node.value)
                }
            }
            println!();
        }
        println!();
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    marked: bool,
    value: usize,
}

pub fn load_game(input: &mut dyn Read) -> (Vec<usize>, Vec<Board>) {
    let reader = BufReader::new(input);
    let mut lines = reader
        .lines()
        .filter(|line| !line.as_ref().unwrap().is_empty());
    let number_string = lines.next().unwrap().unwrap();
    let numbers: Vec<usize> = number_string
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect();

    let mut boards = vec![];

    for chunk in &lines.chunks(5) {
        let rows = chunk
            .map(Result::unwrap)
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse::<usize>().unwrap())
                    .map(|value| Node {
                        value,
                        marked: false,
                    })
                    .collect_vec()
            })
            .collect_vec();

        boards.push(Board { rows })
    }

    (numbers, boards)
}
