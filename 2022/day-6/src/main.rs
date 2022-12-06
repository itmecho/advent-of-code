use itertools::Itertools;

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let answer_1 = marker_end_index(INPUT, 4);
    println!("part 1: {answer_1}");
    let answer_2 = marker_end_index(INPUT, 14);
    println!("part 2: {answer_2}");
}

fn marker_end_index(input: &str, marker_size: usize) -> usize {
    let windows = input.as_bytes().windows(marker_size);
    for (i, window) in windows.enumerate() {
        if window.iter().unique().count() == marker_size {
            return i + marker_size;
        }
    }
    unreachable!();
}

#[cfg(test)]
mod test {
    #[test]
    fn marker_end_index() {
        let cases = vec![
            //part 1
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4, 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 4, 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 4, 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4, 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4, 11),
            // part 2
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14, 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 14, 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 14, 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14, 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14, 26),
        ];

        cases
            .into_iter()
            .for_each(|(input, marker_size, expected)| {
                let actual = super::marker_end_index(input, marker_size);
                assert_eq!(
                    expected, actual,
                    "input: {input}, expected {expected}, got {actual}"
                );
            });
    }
}
