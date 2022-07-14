use std::io::{BufRead, BufReader};

const INPUT: &'static [u8] = include_bytes!("../input.txt");

fn main() {
    let input = BufReader::new(INPUT);
    let output = part1(input);
    println!("part 1: {output}");

    let input = BufReader::new(INPUT);
    let output = part2(input);
    println!("part 2: {output}");
}

fn matching(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => unreachable!(),
    }
}

fn syntax_score(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn autocomplete_score(c: char) -> usize {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!(),
    }
}

fn part1<R: BufRead>(input: R) -> usize {
    let mut score = 0;

    for line in input.lines().map(Result::unwrap) {
        let mut tokens = vec![];

        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => tokens.push(c),
                ')' | ']' | '}' | '>' => {
                    if tokens.pop().unwrap() != matching(c) {
                        // line corrupt
                        score += syntax_score(c);
                        break;
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    score
}

fn part2<R: BufRead>(input: R) -> usize {
    let mut scores = vec![];

    'outer: for line in input.lines().map(Result::unwrap) {
        let mut tokens = vec![];

        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => tokens.push(c),
                ')' | ']' | '}' | '>' => {
                    if tokens.pop().unwrap() != matching(c) {
                        // line corrupt
                        continue 'outer;
                    }
                }
                _ => unreachable!(),
            }
        }

        if tokens.len() > 0 {
            scores.push(
                tokens
                    .into_iter()
                    .rev()
                    .map(matching)
                    .map(autocomplete_score)
                    .fold(0, |acc, v| acc * 5 + v),
            );
        }
    }

    scores.sort();
    *scores.get(scores.len() / 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static [u8] = b"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn part1_test() {
        let input = BufReader::new(INPUT);
        let output = part1(input);
        assert_eq!(26397, output);
    }

    #[test]
    fn part2_test() {
        let input = BufReader::new(INPUT);
        let output = part2(input);
        assert_eq!(288957, output);
    }
}
