use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_input() -> Vec<u16> {
    // test_input()
    real_input()
}

#[allow(dead_code)]
fn test_input() -> Vec<u16> {
    let values = [
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];

    values
        .iter()
        .map(|line| u16::from_str_radix(&line, 2).unwrap())
        .collect()
}

#[allow(dead_code)]
fn real_input() -> Vec<u16> {
    let file = File::open("./input.txt").expect("opening input file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .filter(|line| line.is_ok())
        .map(|line| u16::from_str_radix(&line.unwrap(), 2).expect("parsing input line"))
        .collect()
}
