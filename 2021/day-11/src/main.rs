use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
};

const INPUT: &'static [u8] = include_bytes!("../input.txt");

type Map = [[usize; 10]; 10];

fn dbg_map(m: &Map) {
    for row in 0..10 {
        for col in 0..10 {
            print!("{}", m[row][col]);
        }
        println!("");
    }
}

/// The first value is the X coordinate, the second is the Y
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    pub row: usize,
    pub col: usize,
}

impl Point {
    pub fn relative(&self, x: isize, y: isize) -> Option<Self> {
        let new_x = (self.col as isize) + x;
        let new_y = (self.row as isize) + y;
        if new_x >= 0 && new_x < 10 && new_y >= 0 && new_y < 10 {
            return Some(Self {
                col: new_x as usize,
                row: new_y as usize,
            });
        }
        None
    }
}

fn main() {
    let input = BufReader::new(INPUT);
    let output = part1(input);
    println!("part1 output: {output}");

    let input = BufReader::new(INPUT);
    let output = part2(input);
    println!("part2 output: {output}");
}

fn build_map<R: BufRead>(input: R) -> Map {
    let mut map: Map = [
        [0; 10], [0; 10], [0; 10], [0; 10], [0; 10], [0; 10], [0; 10], [0; 10], [0; 10], [0; 10],
    ];

    for (row, line) in input.lines().map(Result::unwrap).enumerate() {
        for (col, c) in line.chars().enumerate() {
            map[row][col] = c.to_digit(10).unwrap() as usize;
        }
    }

    map
}

type Neighbours = [Option<Point>; 8];

fn neighbours(p: Point) -> Neighbours {
    [
        p.relative(-1, -1),
        p.relative(0, -1),
        p.relative(1, -1),
        p.relative(-1, 0),
        p.relative(1, 0),
        p.relative(-1, 1),
        p.relative(0, 1),
        p.relative(1, 1),
    ]
}

/// Flashes a single point and returns a vec of any other points that should flash
fn flash(map: &mut Map, p: Point) {
    let n = neighbours(p);
    for point in n.into_iter().filter(Option::is_some).map(Option::unwrap) {
        let v = &mut map[point.row][point.col];
        *v += 1;
    }
}

fn step(map: &mut Map) -> usize {
    for row in 0..10 {
        for col in 0..10 {
            map[row][col] += 1;
        }
    }

    let mut flashed = HashSet::new();

    loop {
        let mut flash_count = 0;

        for row in 0..10 {
            for col in 0..10 {
                let point = Point { col, row };
                if map[row][col] > 9 && !flashed.contains(&point) {
                    flashed.insert(point);
                    flash(map, point);
                    flash_count += 1;
                }
            }
        }

        if flash_count == 0 {
            break;
        }
    }

    for p in flashed.iter() {
        map[p.row][p.col] = 0;
    }

    flashed.len()
}

fn part1<R: BufRead>(input: R) -> usize {
    let mut map = build_map(input);
    let mut total_flashed = 0;
    for _ in 0..100 {
        total_flashed += step(&mut map);
    }
    total_flashed
}

fn part2<R: BufRead>(input: R) -> usize {
    let mut map = build_map(input);
    let mut step_number = 0;
    loop {
        step_number += 1;
        if step(&mut map) == 100 {
            return step_number;
        }
    }
}

#[cfg(test)]
mod test {
    use std::io::BufReader;

    use super::*;

    const INPUT: &'static [u8] = b"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn part1_test() {
        let input = BufReader::new(INPUT);
        let output = part1(input);
        assert_eq!(1656, output);
    }

    #[test]
    fn part2_test() {
        let input = BufReader::new(INPUT);
        let output = part2(input);
        assert_eq!(195, output);
    }
}
