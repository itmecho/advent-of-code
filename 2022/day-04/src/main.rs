const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let (a1, a2) = INPUT
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split_once(",").unwrap())
        .map(|(a, b)| (a.split_once("-").unwrap(), b.split_once("-").unwrap()))
        .map(|((a, b), (x, y))| {
            (
                a.parse::<usize>().unwrap(),
                b.parse::<usize>().unwrap(),
                x.parse::<usize>().unwrap(),
                y.parse::<usize>().unwrap(),
            )
        })
        .fold((0, 0), |acc, (a, b, x, y)| {
            (
                acc.0 + ((a <= x && b >= y) || (a >= x && b <= y)) as usize,
                acc.1 + (a <= y && b >= x) as usize,
            )
        });
    println!("part 1: {a1}");
    println!("part 2: {a2}");
}
