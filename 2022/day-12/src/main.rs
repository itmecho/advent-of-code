use std::{
    collections::{BinaryHeap, HashSet},
    fmt::Debug,
    str::FromStr,
};

const INPUT: &str = include_str!("../input.txt");

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Point(usize, usize);

impl Point {
    fn neighbors(&self, width: usize, height: usize) -> Vec<Point> {
        let mut neighbors = vec![];

        // up
        if self.1 > 0 {
            neighbors.push(Point(self.0, self.1 - 1));
        }

        // down
        if self.1 < height - 1 {
            neighbors.push(Point(self.0, self.1 + 1));
        }

        // left
        if self.0 > 0 {
            neighbors.push(Point(self.0 - 1, self.1));
        }

        // right
        if self.0 < width - 1 {
            neighbors.push(Point(self.0 + 1, self.1));
        }

        neighbors
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Node {
    cost: usize,
    point: Point,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point(x: {}, y: {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Map {
    rows: Vec<Vec<u8>>,
    start: Point,
    goal: Point,
    height: usize,
    width: usize,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Self {
            rows: vec![],
            start: Point(0, 0),
            goal: Point(0, 0),
            height: 0,
            width: 0,
        };

        for (rown, line) in s.lines().enumerate() {
            let mut row = vec![];
            let mut push_value = |c: char| {
                row.push(c as u8 - b'a');
            };
            for (coln, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        map.start = Point(coln, rown);
                        push_value('a');
                    }
                    'E' => {
                        map.goal = Point(coln, rown);
                        push_value('z');
                    }
                    _ => push_value(c),
                };
            }
            map.rows.push(row);
        }
        map.height = map.rows.len();
        map.width = map.rows[0].len();
        Ok(map)
    }
}

impl Map {
    fn get(&self, point: Point) -> u8 {
        *self.rows.get(point.1).unwrap().get(point.0).unwrap()
    }
}

fn main() {
    let map = Map::from_str(INPUT).unwrap();

    let answer = bfs(
        &map,
        map.start,
        |p| p == &map.goal,
        |current, neighbor| map.get(*neighbor) <= map.get(*current) + 1,
    )
    .expect("path should have been found");
    println!("part 1: {answer}");

    let answer = bfs(
        &map,
        map.goal,
        |p| map.get(*p) == 0,
        |current, neighbor| map.get(*neighbor) >= map.get(*current) - 1,
    )
    .expect("path should have been found");
    println!("part 1: {answer}");
}

// Breadth first search function that takes in a Map, a start point, a function to determine if the
// current point is the one you're looking for, and a filter function to filter neighbors for the
// current point in the search.
fn bfs<S, F>(map: &Map, start: Point, success: S, filter: F) -> Option<usize>
where
    S: Fn(&Point) -> bool,
    F: Fn(&Point, &Point) -> bool,
{
    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue: BinaryHeap<Node> = BinaryHeap::new();

    queue.push(Node {
        cost: 0,
        point: start,
    });
    visited.insert(start);

    while let Some(Node { cost, point }) = queue.pop() {
        if success(&point) {
            return Some(cost);
        }

        let neighbors: Vec<Point> = point
            .neighbors(map.width, map.height)
            .into_iter()
            .filter(|p| filter(&point, p))
            .collect();

        for neighbor in neighbors {
            if visited.insert(neighbor) {
                queue.push(Node {
                    point: neighbor,
                    cost: cost + 1,
                });
            }
        }
    }

    None
}
