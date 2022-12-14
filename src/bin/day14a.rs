use itertools::Itertools;
use std::fs;

fn get_path_coordinates(coordinate1: &[usize], coordinate2: &[usize]) -> Vec<(usize, usize)> {
    if coordinate1[0] == coordinate2[0] {
        (coordinate1[1].min(coordinate2[1])..=(coordinate1[1].max(coordinate2[1])))
            .map(|y| (coordinate1[0], y))
            .collect()
    } else {
        (coordinate1[0].min(coordinate2[0])..=(coordinate1[0].max(coordinate2[0])))
            .map(|x| (x, coordinate1[1]))
            .collect()
    }
}

fn compute(content: String) -> String {
    let sand_entry = (500, 0);
    let paths: Vec<Vec<Vec<usize>>> = content
        .trim_end()
        .split('\n')
        .map(|path| {
            path.split(" -> ")
                .map(|coordinate| coordinate.split(',').map(|v| v.parse().unwrap()).collect())
                .collect()
        })
        .collect();
    let cave_shape = paths.iter().fold((0, 0), |acc, path| {
        path.iter().fold(acc, |acc, coordinate| {
            (acc.0.max(coordinate[0]), acc.1.max(coordinate[1]))
        })
    });
    let mut cave = vec![vec![false; cave_shape.1 + 1]; cave_shape.0 + 1];
    // Initialize the paths in the cave
    for path in paths {
        for (coordinate1, coordinate2) in path.iter().tuple_windows() {
            for coordinate in get_path_coordinates(coordinate1, coordinate2) {
                cave[coordinate.0][coordinate.1] = true;
            }
        }
    }
    let mut total_sand_unit = 0;
    'outer: loop {
        let mut sand = sand_entry;
        loop {
            if sand.0 == cave_shape.0 || sand.1 == cave_shape.1 {
                break 'outer;
            } else if cave[sand.0][sand.1 + 1]
                && cave[sand.0 - 1][sand.1 + 1]
                && cave[sand.0 + 1][sand.1 + 1]
            {
                cave[sand.0][sand.1] = true;
                total_sand_unit += 1;
                continue 'outer;
            } else if !cave[sand.0][sand.1 + 1] {
                sand.1 += 1;
            } else if !cave[sand.0 - 1][sand.1 + 1] {
                sand = (sand.0 - 1, sand.1 + 1);
            } else if !cave[sand.0 + 1][sand.1 + 1] {
                sand = (sand.0 + 1, sand.1 + 1);
            }
        }
    }
    total_sand_unit.to_string()
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let result = compute(content);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn examples() {
        let content = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";
        let expected = "24";
        assert_eq!(compute(content.to_string()), expected);
    }
}
