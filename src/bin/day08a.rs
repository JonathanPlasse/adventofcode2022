use itertools::min;
use std::fs;

fn compute(content: String) -> String {
    let tree = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut nb_visible_tree = (tree.len() + tree[0].len() - 2) * 2;
    for i in 1..(tree.len() - 1) {
        for j in 1..(tree[0].len() - 1) {
            let left = tree[i][..j].iter().copied().max().unwrap();
            let right = tree[i][(j + 1)..].iter().copied().max().unwrap();
            let top = tree.iter().take(i).map(|t| t[j]).max().unwrap();
            let bottom = tree
                .iter()
                .rev()
                .take(tree.len() - 1 - i)
                .map(|t| t[j])
                .max()
                .unwrap();
            let visibility = min([right, left, top, bottom]).unwrap();
            if visibility < tree[i][j] {
                nb_visible_tree += 1;
            }
        }
    }
    nb_visible_tree.to_string()
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
        let content = "30373
25512
65332
33549
35390
";
        let expected = "21";
        assert_eq!(compute(content.to_string()), expected);
    }
}
