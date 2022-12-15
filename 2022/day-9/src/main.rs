use std::{collections::HashSet, str::FromStr};

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Dir {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => panic!("unsupported direction"),
        })
    }
}

type Move = (Dir, usize);

#[derive(Debug, Default, Hash, Eq, PartialEq, Clone, Copy)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn mv(&mut self, dir: Dir) {
        match dir {
            Dir::Up => self.y += 1,
            Dir::Down => self.y -= 1,
            Dir::Left => self.x -= 1,
            Dir::Right => self.x += 1,
        };
    }

    fn calculate_tail_move(&self, target: Self) -> Self {
        let x_diff = target.x - self.x;
        let y_diff = target.y - self.y;
        let x_diff_abs = x_diff.abs();
        let y_diff_abs = y_diff.abs();

        let mut diff = Self::default();

        if x_diff_abs <= 1 && y_diff_abs <= 1 {
            return diff;
        }

        if x_diff_abs <= 2 && y_diff_abs <= 2 {
            // If we're withing 2 rows and cols, move diagonally
            diff.x = x_diff.clamp(-1, 1);
            diff.y = y_diff.clamp(-1, 1);
        } else if x_diff_abs == 2 && y_diff == 0 {
            // If we're two cols out but in the same row, move over 1 col
            diff.x = x_diff.clamp(-1, 1);
        } else if x_diff == 0 && y_diff_abs == 2 {
            // If we're two rows out but in the same col, move up/down 1 row
            diff.y = y_diff.clamp(-1, 1);
        }

        diff
    }

    fn update(&mut self, target: Self) {
        let diff = self.calculate_tail_move(target);

        self.x += diff.x;
        self.y += diff.y;
    }
}

fn main() {
    let moves: Vec<Move> = INPUT
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(dir, count)| (dir.parse().unwrap(), count.parse().unwrap()))
        .collect();

    let part1_snake = [Pos::default(); 2];
    println!("part 1: {}", process_snake(part1_snake, &moves));
    let part2_snake = [Pos::default(); 10];
    println!("part 2: {}", process_snake(part2_snake, &moves));
}

fn process_snake<T: AsMut<[Pos]>>(mut snake: T, moves: &[Move]) -> usize {
    let snake = snake.as_mut();
    let tail_idx = snake.len() - 1;
    let mut tail_locations = HashSet::new();

    for m in moves {
        for _ in 0..m.1 {
            snake[0].mv(m.0);
            for i in 1..snake.len() {
                let target = snake[i - 1];
                snake[i].update(target);
            }
            tail_locations.insert(snake[tail_idx]);
        }
    }

    tail_locations.len()
}
