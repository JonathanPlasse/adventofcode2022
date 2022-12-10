use std::collections::HashSet;
use std::fs;
use std::ops;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Position(i32, i32);

impl ops::AddAssign<(i32, i32)> for Position {
    fn add_assign(&mut self, rhs: (i32, i32)) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl ops::Sub<Position> for Position {
    type Output = (i32, i32);

    fn sub(self, rhs: Position) -> Self::Output {
        (self.0 - rhs.0, self.1 - rhs.1)
    }
}

fn compute(content: String) -> String {
    let mut head_position = Position(0, 0);
    let mut tail_position = Position(0, 0);
    let mut visited_positions = HashSet::new();
    visited_positions.insert(tail_position);

    for line in content.lines() {
        let (direction, distance) = line.split_once(' ').unwrap();
        let distance: i32 = distance.parse().unwrap();
        for _ in 0..distance {
            head_position += match direction {
                "D" => (1, 0),
                "U" => (-1, 0),
                "R" => (0, 1),
                "L" => (0, -1),
                _ => panic!("unexpected direction"),
            };
            tail_position += match head_position - tail_position {
                (2, 0) => (1, 0),
                (-2, 0) => (-1, 0),
                (0, 2) => (0, 1),
                (0, -2) => (0, -1),
                (1, 2) | (2, 1) | (2, 2) => (1, 1),
                (-1, 2) | (-2, 1) | (-2, 2) => (-1, 1),
                (1, -2) | (2, -1) | (2, -2) => (1, -1),
                (-1, -2) | (-2, -1) | (-2, -2) => (-1, -1),
                (0, 0)
                | (1, 0)
                | (-1, 0)
                | (0, 1)
                | (0, -1)
                | (1, 1)
                | (-1, 1)
                | (1, -1)
                | (-1, -1) => (0, 0),
                _ => panic!("rope too stretched"),
            };
            visited_positions.insert(tail_position);
        }
    }

    visited_positions.len().to_string()
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
    fn examples() {
        let content = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";
        let expected = "13";
        assert_eq!(compute(content.to_string()), expected);
    }
}
