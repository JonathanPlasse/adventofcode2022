use std::collections::BTreeSet;
use std::fs;

fn compute(content: String) -> String {
    let message_length = 4;
    content
        .chars()
        .collect::<Vec<_>>()
        .windows(message_length)
        .enumerate()
        .find(|(_, c)| {
            let unique_char = BTreeSet::from_iter(c.iter().copied());
            unique_char.len() == message_length
        })
        .map(|(i, _)| (i + message_length).to_string())
        .unwrap()
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
        let contents = [
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ];
        let expecteds = ["5", "6", "10", "11"];
        for (content, expected) in contents.iter().zip(expecteds) {
            assert_eq!(compute(content.to_string()), expected);
        }
    }
}
