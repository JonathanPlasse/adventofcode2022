use itertools::Itertools;
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
        .map(|line| line.as_bytes())
        .chunks(3)
        .into_iter()
        .filter_map(|rucksacks| {
            rucksacks
                .fold(None, |acc, rucksack| -> Option<HashSet<u8>> {
                    let items = rucksack.iter().copied().collect();
                    Some(
                        acc.map(|acc| acc.intersection(&items).copied().collect())
                            .unwrap_or(items),
                    )
                })?
                .into_iter()
                .next()
        })
        .map(|item| priority(item) as u32)
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
        let expected = 70;
        assert_eq!(compute(content), expected);
    }
}
