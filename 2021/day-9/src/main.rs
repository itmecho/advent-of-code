use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
};

const INPUT: &[u8] = include_bytes!("../input.txt");

fn main() {
    let input = BufReader::new(INPUT);
    let output = part1(input);
    println!("part 1 output: {}", output);

    let input = BufReader::new(INPUT);
    let output = part2(input);
    println!("part 2 output: {}", output);
}

type Point = (usize, usize);
type Grid = Vec<Vec<usize>>;

fn neighbors(data: &Grid, point: Point) -> [Option<Point>; 4] {
    let (row, col) = point;

    [
        // up
        if row == 0 { None } else { Some((row - 1, col)) },
        // down
        if row == data.len() - 1 {
            None
        } else {
            Some((row + 1, col))
        },
        // left
        if col == 0 { None } else { Some((row, col - 1)) },
        // right
        if col == data[row].len() - 1 {
            None
        } else {
            Some((row, col + 1))
        },
    ]
}

fn build_map<R: BufRead>(input: R) -> Vec<Vec<usize>> {
    input
        .lines()
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .map(|line| {
            line.chars()
                .map(|v| v.to_digit(10).expect("failed to convert to digit") as usize)
                .collect::<Vec<usize>>()
        })
        .collect()
}

fn get_value(data: &Grid, point: Point) -> usize {
    data[point.0][point.1]
}

fn is_low_point(data: &Grid, point: Point) -> bool {
    let val = get_value(data, point);

    neighbors(data, point)
        .into_iter()
        .filter(Option::is_some)
        .all(|p| get_value(data, p.unwrap()) > val)
}

fn get_low_points(data: &Grid) -> Vec<Point> {
    let row_count = data.len();
    let col_count = data[0].len();
    let mut low_points = vec![];

    for row in 0..row_count {
        for col in 0..col_count {
            let point = (row, col);
            if is_low_point(&data, point) {
                low_points.push(point);
            }
        }
    }

    low_points
}

fn basin(data: &Grid, point: Point) -> HashSet<Point> {
    let val = get_value(data, point);
    let mut set = HashSet::from([point]);
    neighbors(data, point)
        .into_iter()
        .filter(Option::is_some)
        .map(Option::unwrap)
        .for_each(|p| {
            let v = get_value(data, p);
            if v != 9 && v > val {
                set.extend(basin(data, p));
            }
        });

    set
}

fn part1<R: BufRead>(input: R) -> usize {
    let data = build_map(input);

    let lp = get_low_points(&data);
    lp.into_iter()
        .map(|p| get_value(&data, p))
        .map(|x| x + 1)
        .sum()
}

fn part2<R: BufRead>(input: R) -> usize {
    let data = build_map(input);

    let lp = get_low_points(&data);
    let mut basins: Vec<usize> = lp.into_iter().map(|p| basin(&data, p).len()).collect();

    basins.sort();
    basins.into_iter().rev().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    const INPUT: &[u8] = b"2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn part1_test_input() {
        let input = BufReader::new(INPUT);
        let output = part1(input);
        assert_eq!(15, output)
    }

    #[test]
    fn part2_test_input() {
        let input = BufReader::new(INPUT);
        let output = part2(input);
        assert_eq!(1134, output)
    }
}
