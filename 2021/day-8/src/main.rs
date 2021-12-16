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
        dbg!(&line);
        let (input, output) = line.split_once('|').unwrap();
        let char_map = build_char_map(input.trim());
        for display in output.trim().split_whitespace() {
            let value = parse_actual_display(display, &char_map);
            match value {
                ZERO => {}
                ONE => total += 1,
                TWO => total += 2,
                THREE => total += 3,
                FOUR => total += 4,
                FIVE => total += 5,
                SIX => total += 6,
                SEVEN => total += 7,
                EIGHT => total += 8,
                NINE => total += 9,
                v => panic!("invalid display value: {:08b}", v),
            };
        }
    }

    println!("Answer: {}", total);
}

fn build_char_map(input: &str) -> HashMap<char, u8> {
    let mut displays = [0u8; 10];
    let mut unknown = vec![];
    for value in input.split_whitespace().map(parse_display) {
        match value.count_ones() {
            2 => displays[1] = value,
            3 => displays[7] = value,
            4 => displays[4] = value,
            7 => displays[8] = value,
            _ => unknown.push(value),
        }
    }

    unknown.retain(|value| {
        if (value ^ displays[8]).count_ones() == 1 {
            if value | displays[4] == *value {
                // Must be 9 as the missing bit isn't in 4
                displays[9] = *value;
            } else if value | displays[1] == *value {
                // Must be 0 as the missing bit is in 4 but not 1
                displays[0] = *value;
            } else {
                // Process of elimination means it must be 6
                displays[6] = *value;
            }
            false
        } else {
            true
        }
    });

    // Now I just need to figure out 2, 3, and 5
    unknown.retain(|value| {
        if *value == displays[6] & displays[9] {
            // If the 2 missing bits are the missing bits from 6 and 9, it matches 5
            displays[5] = *value
        } else if (value | displays[9]).count_zeros() == 1 {
            // Otherwise if there is only one bit missing when combined with 9, it must be 2
            displays[2] = *value
        } else {
            // Otherwise it must be a 3 because that is all that's left!
            displays[3] = *value
        }
        false
    });

    assert!(unknown.len() == 0);

    let mut char_map = HashMap::new();
    char_map.insert('a', displays[1] ^ displays[7]);
    char_map.insert('b', (displays[2] | displays[1]) ^ displays[8]);
    char_map.insert('c', displays[6] ^ displays[8]);
    char_map.insert('d', displays[0] ^ displays[8]);
    char_map.insert('e', displays[9] ^ displays[8]);
    char_map.insert('f', !displays[2] & displays[1]);
    char_map.insert(
        'g',
        !char_map.iter().fold(0u8, |acc, (_, value)| acc | value) ^ 1,
    );
    for (k, v) in &char_map {
        println!("{}: {:08b}", k, v);
    }

    char_map
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

fn parse_actual_display(display: &str, char_map: &HashMap<char, u8>) -> u8 {
    let mut actual_display = String::new();
    for c in display.chars() {
        let new_char = match *char_map.get(&c).unwrap() {
            A => 'a',
            B => 'b',
            C => 'c',
            D => 'd',
            E => 'e',
            F => 'f',
            G => 'g',
            _ => panic!("dodgy input"),
        };
        println!("{} -> {}", c, new_char);
        actual_display.push(new_char);
    }
    parse_display(&actual_display)
}
