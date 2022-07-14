#![feature(int_abs_diff)]
use std::io::Read;

fn main() {
    let mut cli = common::Cli::new(part_1, part_2);
    cli.run();
}

fn part_1(input_file: &mut dyn Read) {
    let mut input = process_input(input_file);
    input.sort();
    let median = find_median(&input);
    let mut moves = 0;

    for value in input.iter() {
        moves += value.abs_diff(median);
    }

    println!("Answer: {}", moves);
}

fn part_2(input_file: &mut dyn Read) {
    let mut input = process_input(input_file);
    input.sort();
    let sum: f64 = input.iter().sum::<usize>() as f64;
    let mean: f64 = sum / input.len() as f64;
    let mean = mean.ceil() as usize;

    let mut amoves = 0;
    for value in input.iter() {
        amoves += sigma(*value, mean);
    }

    let mut bmoves = 0;
    for value in input.iter() {
        bmoves += sigma(*value, mean - 1);
    }

    println!("Answer: {}", amoves.min(bmoves));
}

fn process_input(input_file: &mut dyn Read) -> Vec<usize> {
    let mut data = String::new();
    input_file
        .read_to_string(&mut data)
        .expect("reading input file");

    let mut values: Vec<usize> = data
        .split(',')
        .into_iter()
        .map(str::trim)
        .map(|v| v.parse().expect("parsing input number"))
        .collect();

    values.sort();
    values
}

fn find_median(values: &[usize]) -> usize {
    let median = values.get(values.len() / 2).unwrap();
    println!("found median: {}", median);
    *median
}

fn sigma(start: usize, target: usize) -> usize {
    let mut total = 0;

    for i in 1..=start.abs_diff(target) {
        total += i;
    }

    total
}
