use std::io::{BufRead, BufReader, Read};

fn main() {
    let mut cli = common::Cli::new(part_1, part_2);
    cli.run();
}

fn part_1(input: &mut dyn Read) {
    let reader = BufReader::new(input);
    let input = reader
        .lines()
        .filter(|line| line.is_ok())
        .map(|line| u16::from_str_radix(&line.unwrap(), 2).expect("parsing input line"))
        .collect::<Vec<u16>>();

    let threshold = input.len() / 2;
    let mut counts = [0; 16];

    for value in input {
        for idx in 0..16 {
            if value & (1 << (15 - idx)) > 0 {
                counts[idx] = counts[idx] + 1
            }
        }
    }

    let mut gamma_rate: u16 = 0;
    let mut epsilon: u16 = 0;

    for idx in 0..16 {
        if counts[idx] == 0 {
            continue;
        }

        if counts[idx] > threshold {
            gamma_rate = gamma_rate | 1 << (15 - idx);
        } else {
            epsilon = epsilon | 1 << (15 - idx);
        }
    }

    println!("Answer: {}", gamma_rate as u32 * epsilon as u32);
}

fn part_2(input: &mut dyn Read) {
    let reader = BufReader::new(input);
    let input = reader
        .lines()
        .filter(|line| line.is_ok())
        .map(|line| u16::from_str_radix(&line.unwrap(), 2).expect("parsing input line"))
        .collect::<Vec<u16>>();

    let mut counts = [0; 16];

    for value in &input {
        for idx in 0..16 {
            if value & (1 << (15 - idx)) > 0 {
                counts[idx] = counts[idx] + 1
            }
        }
    }

    let mut oxygen = input.clone();
    let mut co2 = input.clone();

    for idx in 0..16 {
        let bitmask = 1 << (15 - idx);
        if counts[idx] == 0 {
            continue;
        }

        if oxygen.len() > 1 {
            filter_values(&mut oxygen, bitmask, |value| value & bitmask > 0)
        }

        if co2.len() > 1 {
            filter_values(&mut co2, bitmask, |value| value & bitmask == 0)
        }
    }

    if oxygen.len() != 1 {
        panic!("found multiple oxygen measurements");
    }

    let oxygen_rate = oxygen.first().unwrap();

    if co2.len() != 1 {
        panic!("found multiple oxygen measurements");
    }

    let co2_rate = co2.first().unwrap();

    println!("Answer: {}", *oxygen_rate as u64 * *co2_rate as u64);
}

fn get_count(input: &Vec<u16>, bitmask: u16) -> usize {
    let mut count = 0;
    for value in input {
        if value & bitmask > 0 {
            count = count + 1;
        }
    }
    count
}

fn filter_values<F: FnMut(&u16) -> bool>(input: &mut Vec<u16>, bitmask: u16, mut condition: F) {
    let ones = get_count(&input, bitmask);
    if ones == 0 {
        return;
    }

    let zeros = input.len() - ones;

    if ones >= zeros {
        input.retain(condition);
    } else {
        input.retain(|value| !condition(value));
    }
}
