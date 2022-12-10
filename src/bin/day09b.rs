use std::collections::HashSet;
use std::fs;
use std::ops;

const ROPE_LENGTH: usize = 10;

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
    let mut rope = [Position(0, 0); ROPE_LENGTH];
    let mut visited_positions = HashSet::new();
    visited_positions.insert(rope[9]);

    for line in content.lines() {
        let (direction, distance) = line.split_once(' ').unwrap();
        let distance: i32 = distance.parse().unwrap();
        for _ in 0..distance {
            rope[0] += match direction {
                "D" => (1, 0),
                "U" => (-1, 0),
                "R" => (0, 1),
                "L" => (0, -1),
                _ => panic!("unexpected direction"),
            };
            for i in 0..(ROPE_LENGTH - 1) {
                rope[i + 1] += match rope[i] - rope[i + 1] {
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
            }
            visited_positions.insert(rope[ROPE_LENGTH - 1]);
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
        let content = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
        let expected = "36";
        assert_eq!(compute(content.to_string()), expected);
    }
}
