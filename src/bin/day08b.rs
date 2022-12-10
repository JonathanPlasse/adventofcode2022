use std::fs;

fn count_visible_trees(trees: impl Iterator<Item = u8>, current_tree_hight: u8) -> u32 {
    let mut nb_visible_tree = 0;
    for tree in trees {
        nb_visible_tree += 1;
        if tree >= current_tree_hight {
            break;
        }
    }
    nb_visible_tree
}

fn compute(content: String) -> String {
    let tree = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut max_scenic_score = 0;
    for i in 0..tree.len() {
        for j in 0..tree[0].len() {
            let left_trees = tree[i][..j].iter().copied().rev();
            let right_trees = tree[i][(j + 1)..].iter().copied();
            let top_trees = tree.iter().take(i).map(|t| t[j]).rev();
            let bottom_trees = tree
                .iter()
                .rev()
                .take(tree.len() - 1 - i)
                .rev()
                .map(|t| t[j]);

            let trees: [Box<dyn Iterator<Item = u8>>; 4] = [
                Box::new(left_trees),
                Box::new(right_trees),
                Box::new(top_trees),
                Box::new(bottom_trees),
            ];

            max_scenic_score = max_scenic_score.max(
                trees
                    .into_iter()
                    .map(|trees| count_visible_trees(trees, tree[i][j]))
                    .product(),
            );
        }
    }
    max_scenic_score.to_string()
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
        let expected = "8";
        assert_eq!(compute(content.to_string()), expected);
    }
}
