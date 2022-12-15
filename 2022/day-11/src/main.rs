use std::{cell::RefCell, collections::VecDeque, fmt::Display, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone)]
enum Op {
    Add(usize),
    Mul(usize),
    MulOld,
    AddOld,
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();

        Ok(match parts[1] {
            "+" => match parts[2] {
                "old" => Self::AddOld,
                value => Self::Add(value.parse().unwrap()),
            },
            "*" => match parts[2] {
                "old" => Self::MulOld,
                value => Self::Mul(value.parse().unwrap()),
            },
            _ => panic!("unsupported operation: {s}"),
        })
    }
}

impl Op {
    fn perform(&self, value: usize) -> usize {
        match self {
            Self::Add(op_value) => value + op_value,
            Self::AddOld => value + value,
            Self::Mul(op_value) => value * op_value,
            Self::MulOld => value * value,
        }
    }
}

#[derive(Debug, Clone)]
struct Test {
    div: usize,
    success: usize,
    fail: usize,
}

#[derive(Debug, Clone)]
struct Monkey {
    id: usize,
    items: VecDeque<usize>,
    op: Op,
    test: Test,
    inspection_count: usize,
}

lazy_static! {
    static ref ID_PATTERN: Regex = Regex::new(r"Monkey (\d+):").unwrap();
    static ref OP_PATTERN: Regex = Regex::new(r"Operation: new = (.+)").unwrap();
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let id_line = lines.next().unwrap();
        let id = ID_PATTERN
            .captures(id_line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap();

        let items = lines
            .next()
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .split(",")
            .map(|i| i.trim().parse::<usize>().unwrap())
            .collect();

        let op_line = lines.next().unwrap();
        let op_str = OP_PATTERN
            .captures(op_line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();
        let op = Op::from_str(op_str).unwrap();

        let test_div = lines
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let test_true = lines
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let test_false = lines
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let test = Test {
            div: test_div,
            success: test_true,
            fail: test_false,
        };

        Ok(Monkey {
            id,
            items,
            op,
            test,
            inspection_count: 0,
        })
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items = self
            .items
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "Monkey {}: {}", self.id, items)
    }
}

type Monkeys = Vec<RefCell<Monkey>>;
fn main() {
    let monkeys: (Monkeys, Monkeys) = INPUT
        .split("\n\n")
        .map(Monkey::from_str)
        .map(Result::unwrap)
        .fold((vec![], vec![]), |mut acc, m| {
            acc.0.push(RefCell::new(m.clone()));
            acc.1.push(RefCell::new(m));
            acc
        });

    // I usually read fasterthanlime's article after I've finished the puzzle
    // but the maths in part 2 of this one is beyond me so I had to read it
    // after getting part 1.
    // https://fasterthanli.me/series/advent-of-code-2022/part-11#part-2
    let divisor_product = monkeys
        .0
        .iter()
        .map(|m| m.borrow().test.div)
        .product::<usize>();

    println!("part 1: {}", calculate_monkey_business(monkeys.0, 20, None));
    println!(
        "part 2: {}",
        calculate_monkey_business(monkeys.1, 10000, Some(divisor_product))
    );
}

fn calculate_monkey_business(
    monkeys: Monkeys,
    rounds: usize,
    heightened_worry: Option<usize>,
) -> usize {
    for _ in 1..=rounds {
        for monkey_id in 0..monkeys.len() {
            let mut monkey = monkeys.get(monkey_id).unwrap().borrow_mut();

            while let Some(item) = monkey.items.pop_front() {
                monkey.inspection_count += 1;
                let mut item = monkey.op.perform(item);
                if heightened_worry.is_some() {
                    // This is the bit that I needed from fasterthanlime!
                    item %= heightened_worry.unwrap();
                } else {
                    item /= 3;
                }
                let mut target = if item % monkey.test.div == 0 {
                    monkeys.get(monkey.test.success).unwrap().borrow_mut()
                } else {
                    monkeys.get(monkey.test.fail).unwrap().borrow_mut()
                };

                target.items.push_back(item);
            }
        }
    }

    // TODO: get top two from tracker and multiply counts
    let mut counts: Vec<usize> = monkeys
        .iter()
        .map(|m| m.borrow().inspection_count)
        .collect();
    counts.sort();
    counts.reverse();

    counts[0] * counts[1]
}
