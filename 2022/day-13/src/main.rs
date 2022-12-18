use std::{cmp::Ordering, fmt};

use serde::Deserialize;

// This is fasterthanlime's solution
const INPUT: &str = include_str!("../input.txt");

#[derive(Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
enum Node {
    Number(usize),
    List(Vec<Self>),
}

impl Node {
    fn with_slice<T>(&self, f: impl FnOnce(&[Node]) -> T) -> T {
        match self {
            Self::List(n) => f(&n[..]),
            Self::Number(n) => f(&[Self::Number(*n)]),
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::List(n) => f.debug_list().entries(n).finish(),
        }
    }
}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Node::Number(a), Node::Number(b)) => a.partial_cmp(b),
            (l, r) => Some(l.with_slice(|l| {
                r.with_slice(|r| {
                    l.iter()
                        .zip(r.iter())
                        .map(|(aa, bb)| aa.cmp(bb))
                        // return the first ordering that isn't `Equal`
                        .find(|&ord| ord != Ordering::Equal)
                        // or compare the lengths
                        .unwrap_or_else(|| l.len().cmp(&r.len()))
                })
            })),
        }
    }
}

impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let mut sum = 0;
    for (i, groups) in input.split("\n\n").enumerate() {
        let i = i + 1;

        let mut nodes = groups
            .lines()
            .map(|line| serde_json::from_str::<Node>(line).unwrap());
        let l = nodes.next().unwrap();
        let r = nodes.next().unwrap();
        if l < r {
            sum += i;
        }
    }
    sum
}

// This part isn't fasterthanlime's!
fn part2(input: &str) -> usize {
    let dividers = vec![
        Node::List(vec![Node::Number(2)]),
        Node::List(vec![Node::Number(6)]),
    ];

    let mut packets: Vec<Node> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    packets.extend(dividers.clone());

    packets.sort();

    let location_1 = packets.binary_search(&dividers[0]).unwrap() + 1;
    let location_2 = packets.binary_search(&dividers[1]).unwrap() + 1;

    location_1 * location_2
}
