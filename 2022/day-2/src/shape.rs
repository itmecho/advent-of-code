use std::{cmp::Ordering, str::FromStr};

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl FromStr for Shape {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(format!("unknown symbol: {s}")),
        }
    }
}

impl Eq for Shape {}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Shape {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Self::Rock => match other {
                Self::Rock => Ordering::Equal,
                Self::Paper => Ordering::Less,
                Self::Scissors => Ordering::Greater,
            },
            Self::Paper => match other {
                Self::Rock => Ordering::Greater,
                Self::Paper => Ordering::Equal,
                Self::Scissors => Ordering::Less,
            },
            Self::Scissors => match other {
                Self::Rock => Ordering::Less,
                Self::Paper => Ordering::Greater,
                Self::Scissors => Ordering::Equal,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::Shape;

    #[test]
    fn from_str() {
        let mut cases = HashMap::new();
        cases.insert("A", Shape::Rock);
        cases.insert("X", Shape::Rock);
        cases.insert("B", Shape::Paper);
        cases.insert("Y", Shape::Paper);
        cases.insert("C", Shape::Scissors);
        cases.insert("Z", Shape::Scissors);

        cases.iter().for_each(|(input, expected)| {
            let actual: Shape = input.parse().expect("should parse");
            assert_eq!(*expected, actual);
        });
    }

    #[test]
    fn cmp() {
        assert!(Shape::Rock < Shape::Paper);
        assert!(Shape::Rock == Shape::Rock);
        assert!(Shape::Rock > Shape::Scissors);
        assert!(Shape::Paper == Shape::Paper);
        assert!(Shape::Paper > Shape::Rock);
        assert!(Shape::Paper < Shape::Scissors);
        assert!(Shape::Scissors > Shape::Paper);
        assert!(Shape::Scissors < Shape::Rock);
        assert!(Shape::Scissors == Shape::Scissors);
    }
}
