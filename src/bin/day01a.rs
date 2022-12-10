use std::fs;

fn compute(content: String) -> u32 {
    content
        .split("\n\n")
        .map(|calories| {
            calories
                .lines()
                .map(|calory| calory.parse::<u32>().unwrap())
                .sum()
        })
        .max()
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
    fn example() {
        let content = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
            .to_string();
        let expected = 24000u32;
        assert_eq!(compute(content), expected);
    }
}
