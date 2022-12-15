const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let mut elves = INPUT
        .lines()
        .collect::<Vec<&str>>()
        .split(|s| s.is_empty())
        .map(|s| s.iter().map(|v| v.parse::<usize>().unwrap()).sum())
        .collect::<Vec<usize>>();

    elves.sort();
    elves.reverse();

    println!("part 1: {}", elves[0]);
    println!("part 2: {}", elves.iter().take(3).sum::<usize>());
}
