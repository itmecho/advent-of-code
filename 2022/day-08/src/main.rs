use std::collections::HashSet;

const INPUT: &'static str = include_str!("../input.txt");

type Grid = Vec<Vec<u32>>;
type Point = (usize, usize);

fn main() {
    let grid: Grid = INPUT
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut seen: HashSet<Point> = HashSet::new();
    seen.insert((0, 0));
    seen.insert((grid[0].len() - 1, 0));
    seen.insert((0, grid.len() - 1));
    seen.insert((grid[0].len() - 1, grid.len() - 1));
    for x in 1..grid[0].len() - 1 {
        part1_look(&grid, &mut seen, (x, 0), Direction::South);
        part1_look(&grid, &mut seen, (x, grid.len() - 1), Direction::North);
    }

    for y in 1..grid.len() - 1 {
        part1_look(&grid, &mut seen, (0, y), Direction::East);
        part1_look(&grid, &mut seen, (grid[0].len() - 1, y), Direction::West);
    }

    println!("part 1: {}", seen.len());

    let mut highest_score = 0;
    for x in 0..=grid[0].len() - 1 {
        for y in 0..=grid.len() - 1 {
            let pos = (x, y);
            let n = part2_look(&grid, pos, Direction::North);
            let e = part2_look(&grid, pos, Direction::East);
            let s = part2_look(&grid, pos, Direction::South);
            let w = part2_look(&grid, pos, Direction::West);
            let score = n * e * s * w;
            if score > highest_score {
                highest_score = score;
            }
        }
    }
    println!("part 2: {highest_score}");
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn advance(grid: &Grid, (x, y): Point, direction: Direction) -> Option<(usize, usize)> {
    let row_count = grid.len() - 1;
    let col_count = grid[0].len() - 1;
    match direction {
        Direction::North => {
            if y == 0 {
                None
            } else {
                Some((x, y - 1))
            }
        }
        Direction::East => {
            if x == col_count {
                None
            } else {
                Some((x + 1, y))
            }
        }
        Direction::South => {
            if y == row_count {
                None
            } else {
                Some((x, y + 1))
            }
        }
        Direction::West => {
            if x == 0 {
                None
            } else {
                Some((x - 1, y))
            }
        }
    }
}

fn part1_look(
    grid: &Grid,
    seen: &mut HashSet<Point>,
    mut pos: (usize, usize),
    direction: Direction,
) {
    let mut tallest = grid[pos.1][pos.0];
    seen.insert(pos);

    loop {
        let next_pos = advance(grid, pos, direction);
        if next_pos.is_none() {
            break;
        }
        pos = next_pos.unwrap();

        if grid[pos.1][pos.0] > tallest {
            tallest = grid[pos.1][pos.0];
            seen.insert(pos);
        }
        // If we hit the tallest tree, just stop looking
        if tallest == 9 {
            break;
        }
    }
}

fn part2_look(grid: &Grid, mut pos: (usize, usize), direction: Direction) -> usize {
    let height = grid[pos.1][pos.0];
    let mut score = 0;

    loop {
        let next_pos = advance(grid, pos, direction);
        if next_pos.is_none() {
            break;
        }
        pos = next_pos.unwrap();

        score += 1;

        if grid[pos.1][pos.0] >= height {
            break;
        }
    }

    score
}
