use std::fs;

#[derive(Clone, Copy)]
enum RockPaperScissors {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl RockPaperScissors {
    fn new(letter: &str) -> Self {
        match letter {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            _ => panic!(),
        }
    }

    fn stategy(opponent: &str, me: &str) -> Self {
        match (opponent, me) {
            ("A", "X") => Self::Scissors,
            ("B", "X") => Self::Rock,
            ("C", "X") => Self::Paper,
            ("A", "Y") => Self::Rock,
            ("B", "Y") => Self::Paper,
            ("C", "Y") => Self::Scissors,
            ("A", "Z") => Self::Paper,
            ("B", "Z") => Self::Scissors,
            ("C", "Z") => Self::Rock,
            _ => panic!(),
        }
    }

    fn points(opponent: Self, me: Self) -> u32 {
        let points = me as u32;
        points
            + match (opponent, me) {
                (Self::Rock, Self::Paper) => 6,
                (Self::Paper, Self::Scissors) => 6,
                (Self::Scissors, Self::Rock) => 6,
                (Self::Rock, Self::Rock) => 3,
                (Self::Paper, Self::Paper) => 3,
                (Self::Scissors, Self::Scissors) => 3,
                _ => 0,
            }
    }
}

fn compute(content: String) -> u32 {
    content
        .lines()
        .map(|v| v.split(' ').take(2).collect::<Vec<_>>())
        .map(|v| {
            RockPaperScissors::points(
                RockPaperScissors::new(v[0]),
                RockPaperScissors::stategy(v[0], v[1]),
            )
        })
        .sum()
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let result = compute(content);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let content = "\
A Y
B X
C Z
"
        .to_string();
        let expected = 12;
        assert_eq!(compute(content), expected);
    }
}
