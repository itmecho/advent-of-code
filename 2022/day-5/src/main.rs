use regex::Regex;

const INPUT: &'static str = include_str!("../input.txt");

type Stacks = Vec<Vec<char>>;

#[derive(Debug, Clone)]
struct Command {
    count: usize,
    src: usize,
    dest: usize,
}

fn main() {
    let (stacks, commands): (Stacks, Vec<Command>) = INPUT
        .split_once("\n\n")
        .map(|(stacks, commands)| (parse_stacks(stacks), parse_commands(commands)))
        .unwrap();

    part1(stacks.clone(), commands.clone());
    part2(stacks, commands);
}

fn part1(mut stacks: Stacks, commands: Vec<Command>) {
    for cmd in commands {
        for _ in 0..cmd.count {
            let c = stacks.get_mut(cmd.src - 1).unwrap().pop().unwrap();
            stacks.get_mut(cmd.dest - 1).unwrap().push(c);
        }
    }
    let answer: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();
    println!("part 1: {answer}");
}

fn part2(mut stacks: Stacks, commands: Vec<Command>) {
    for cmd in commands {
        let stack = stacks.get_mut(cmd.src - 1).unwrap();
        let mut chunk = stack.split_off(stack.len() - cmd.count);
        stacks.get_mut(cmd.dest - 1).unwrap().append(&mut chunk);
    }
    let answer: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();
    println!("part 2: {answer}");
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let mut iter = input.lines().rev();
    let ids = iter
        .next()
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();
    let mut stacks: Vec<Vec<char>> = vec![vec![]; ids.len()];

    iter.for_each(|s| {
        (0..ids.len())
            .map(|s| s * 4 + 1)
            .enumerate()
            .for_each(|(stack_id, idx)| {
                let c = s.as_bytes()[idx];
                c.is_ascii_alphabetic()
                    .then(|| stacks[stack_id].push(c as char));
            });
    });
    stacks
}

fn parse_commands(input: &str) -> Vec<Command> {
    let pattern: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    input
        .lines()
        .map(|line| pattern.captures(line).unwrap())
        .map(|caps| {
            (
                caps[1].parse().unwrap(),
                caps[2].parse().unwrap(),
                caps[3].parse().unwrap(),
            )
        })
        .map(|(count, src, dest)| Command { count, src, dest })
        .collect()
}
