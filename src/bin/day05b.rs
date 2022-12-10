use itertools::Itertools;
use regex::Regex;
use std::fs;

fn compute(content: String) -> String {
    let nb_stacks: usize = (content.find('\n').unwrap() + 1) / 4;
    let mut stacks = vec![Vec::new(); nb_stacks];

    let stack_pattern = Regex::new({
        let crate_pattern = r"(?:\[([A-Z])\]|   )";
        let mut crates = vec![];
        crates.resize(nb_stacks, crate_pattern);
        format!("^{}$", crates.join(" ")).as_str()
    })
    .unwrap();
    let move_pattern = Regex::new(r"^move (\d.*) from (\d.*) to (\d.*)$").unwrap();

    let (crates, moves) = content.split_once("\n\n").unwrap();

    crates.lines().rev().skip(1).for_each(|line| {
        let cap = stack_pattern.captures_iter(line).next().unwrap();
        for (i, stack) in stacks.iter_mut().enumerate() {
            if let Some(c) = cap.get(i + 1) {
                (*stack).push(c.as_str());
            }
        }
    });
    moves.lines().for_each(|line| {
        let cap = move_pattern.captures_iter(line).next().unwrap();
        let move_nb = cap[1].parse::<usize>().unwrap();
        let from = cap[2].parse::<usize>().unwrap() - 1;
        let to = cap[3].parse::<usize>().unwrap() - 1;
        let mut tmp = vec![];
        for _ in 0..move_nb {
            tmp.push(stacks[from].pop().unwrap());
        }
        stacks[to].extend(tmp.iter().rev());
    });
    stacks.iter().map(|stack| stack[stack.len() - 1]).join("")
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
        let content = "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"
        .to_string();
        let expected = "MCD";
        assert_eq!(compute(content), expected);
    }
}
