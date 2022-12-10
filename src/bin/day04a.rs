use std::fs;

fn compute(content: String) -> usize {
    content
        .lines()
        .map(|ranges| {
            ranges
                .split(&[',', '-'])
                .map(str::parse::<u32>)
                .map(Result::unwrap)
                .collect::<Vec<_>>()
        })
        .filter(|ranges| {
            ranges[0] <= ranges[2] && ranges[3] <= ranges[1]
                || ranges[2] <= ranges[0] && ranges[1] <= ranges[3]
        })
        .count()
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
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"
        .to_string();
        let expected = 2;
        assert_eq!(compute(content), expected);
    }
}
