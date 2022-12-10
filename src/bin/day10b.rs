use std::fs;

use itertools::Itertools;

fn is_pixel_lit(nth_cycle: i32, x: i32) -> bool {
    x - 1 <= (nth_cycle - 1) % 40 && (nth_cycle - 1) % 40 <= x + 1
}

fn compute(content: String) -> String {
    let mut crt = [false; 240];
    let mut nth_cycle = 1;
    let mut x = 1;
    content.lines().for_each(|line| match line.split_once(' ') {
        Some((_, value)) => {
            let value = value.parse::<i32>().unwrap();
            for _ in 0..2 {
                crt[(nth_cycle - 1) as usize] = is_pixel_lit(nth_cycle, x);
                nth_cycle += 1;
            }
            x += value;
        }
        None => {
            crt[(nth_cycle - 1) as usize] = is_pixel_lit(nth_cycle, x);
            nth_cycle += 1;
        }
    });
    crt.chunks(40)
        .map(|w| w.iter().map(|v| if *v { '#' } else { '.' }).join(""))
        .join("\n")
        + "\n"
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
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";
        let expected = "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";
        assert_eq!(compute(content.to_string()), expected);
    }
}
