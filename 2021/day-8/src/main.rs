use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};

const A: u8 = 1 << 7;
const B: u8 = 1 << 6;
const C: u8 = 1 << 5;
const D: u8 = 1 << 4;
const E: u8 = 1 << 3;
const F: u8 = 1 << 2;
const G: u8 = 1 << 1;

const ZERO: u8 = A | B | C | E | F | G;
const ONE: u8 = C | F;
const TWO: u8 = A | C | D | E | G;
const THREE: u8 = A | C | D | F | G;
const FOUR: u8 = B | C | D | F;
const FIVE: u8 = A | B | D | F | G;
const SIX: u8 = A | B | D | E | F | G;
const SEVEN: u8 = A | C | F;
const EIGHT: u8 = A | B | C | D | E | F | G;
const NINE: u8 = A | B | C | D | F | G;

fn main() {
    let mut cli = common::Cli::new(part_1, part_2);
    cli.run();
}

fn part_1(input: &mut dyn Read) {
    let reader = BufReader::new(input);

    let mut simple_count = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let (_, output) = line.split_once('|').unwrap();
        for display in output.trim().split_whitespace() {
            let x = parse_display(display).count_ones();
            if [2, 3, 4, 7].contains(&x) {
                simple_count += 1;
            }
        }
    }

    println!("Answer: {}", simple_count);
}

fn part_2(input: &mut dyn Read) {
    let reader = BufReader::new(input);

    let mut total = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let (input, output) = line.split_once('|').unwrap();
        let display_map = get_display_map(input);

        let mut result = 0;
        for (idx, display) in output.trim().split_whitespace().enumerate() {
            let parsed_display = parse_display(display);
            let converted_display = display_map
                .get(&parsed_display)
                .expect("Somehow failed to end up with this display in the map");

            let display_value = match *converted_display {
                ZERO => 0,
                ONE => 1,
                TWO => 2,
                THREE => 3,
                FOUR => 4,
                FIVE => 5,
                SIX => 6,
                SEVEN => 7,
                EIGHT => 8,
                NINE => 9,
                _ => unreachable!(),
            };

            result += display_value as u32 * 10u32.pow(3 - idx as u32);
        }

        total += result;
    }

    println!("Answer: {}", total);
}

fn get_display_map(input: &str) -> HashMap<u8, u8> {
    let mut display_map: HashMap<u8, u8> = HashMap::new();
    let mut unknown = vec![];
    for value in input.split_whitespace().map(parse_display) {
        match value.count_ones() {
            2 => {
                display_map.insert(ONE, value);
            }
            3 => {
                display_map.insert(SEVEN, value);
            }
            4 => {
                display_map.insert(FOUR, value);
            }
            7 => {
                display_map.insert(EIGHT, value);
            }
            _ => unknown.push(value),
        };
    }

    unknown.retain(|value| {
        if (value ^ display_map.get(&EIGHT).unwrap()).count_ones() == 1 {
            if value | display_map.get(&FOUR).unwrap() == *value {
                // Must be 9 as the missing bit isn't in 4
                display_map.insert(NINE, *value);
            } else if value | display_map.get(&ONE).unwrap() == *value {
                // Must be 0 as the missing bit is in 4 but not 1
                display_map.insert(ZERO, *value);
            } else {
                // Process of elimination means it must be 6
                display_map.insert(SIX, *value);
            }
            false
        } else {
            true
        }
    });

    // Now I just need to figure out 2, 3, and 5
    unknown.retain(|value| {
        if *value == display_map.get(&SIX).unwrap() & display_map.get(&NINE).unwrap() {
            // If the 2 missing bits are the missing bits from 6 and 9, it matches 5
            display_map.insert(FIVE, *value);
        } else if (value | display_map.get(&NINE).unwrap()).count_zeros() == 1 {
            // Otherwise if there is only one bit missing when combined with 9, it must be 2
            display_map.insert(TWO, *value);
        } else {
            // Otherwise it must be a 3 because that is all that's left!
            display_map.insert(THREE, *value);
        }
        false
    });

    let mut inverted_map = HashMap::new();
    for (k, v) in display_map {
        inverted_map.insert(v, k);
    }

    inverted_map
}

fn parse_display(display: &str) -> u8 {
    display.chars().fold(0u8, |acc, c| match c {
        'a' => acc | A,
        'b' => acc | B,
        'c' => acc | C,
        'd' => acc | D,
        'e' => acc | E,
        'f' => acc | F,
        'g' => acc | G,
        c => panic!("unsupported character {}", c),
    })
}
