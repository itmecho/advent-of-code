use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_input() -> Vec<u16> {
    let mut args = std::env::args();
    args.next();

    let file = File::open(&args.next().expect("reading first argument (input file)"))
        .expect("opening input file");

    let reader = BufReader::new(file);
    reader
        .lines()
        .filter(|line| line.is_ok())
        .map(|line| u16::from_str_radix(&line.unwrap(), 2).expect("parsing input line"))
        .collect()
}
