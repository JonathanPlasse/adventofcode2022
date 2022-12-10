use std::collections::HashSet;
use std::fs;

fn compute(content: String) -> u32 {
    fn priority(item: u8) -> u8 {
        if item >= b'a' {
            item - b'a' + 1
        } else {
            item - b'A' + 1 + 26
        }
    }
    content
        .lines()
        .map(|line| {
            let (compartment1, compartment2) = line.as_bytes().split_at(line.len() / 2);
            compartment1
                .iter()
                .collect::<HashSet<_>>()
                .intersection(&compartment2.iter().collect())
                .map(|&&item| priority(item) as u32)
                .next()
                .unwrap()
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
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"
        .to_string();
        let expected = 157;
        assert_eq!(compute(content), expected);
    }
}
