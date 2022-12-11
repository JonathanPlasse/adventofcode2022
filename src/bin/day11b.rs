use num::Integer;
use std::collections::BinaryHeap;
use std::fs;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
enum Operand {
    #[default]
    Old,
    Number(u64),
}

impl Operand {
    fn new(operand: &str) -> Self {
        match operand {
            "old" => Self::Old,
            number => Self::Number(number.parse().unwrap()),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
enum Operator {
    #[default]
    Addition,
    Multiplication,
}

impl Operator {
    fn new(operator: &str) -> Self {
        match operator {
            "+" => Self::Addition,
            "*" => Self::Multiplication,
            _ => panic!("unexpected operator"),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Operation {
    first_operand: Operand,
    second_operand: Operand,
    operator: Operator,
}

impl Operation {
    fn calculate(&self, old: u64) -> u64 {
        let first_operand = match self.first_operand {
            Operand::Number(operand) => operand,
            Operand::Old => old,
        };
        let second_operand = match self.second_operand {
            Operand::Number(operand) => operand,
            Operand::Old => old,
        };
        match self.operator {
            Operator::Addition => first_operand + second_operand,
            Operator::Multiplication => first_operand * second_operand,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Test {
    divisible_by: u64,
    if_true_throw_to_monkey_nb: usize,
    if_false_throw_to_monkey_nb: usize,
}

impl Test {
    fn calculate_which_monkey_to_throw(&self, worry_level: u64) -> usize {
        if worry_level % self.divisible_by == 0 {
            self.if_true_throw_to_monkey_nb
        } else {
            self.if_false_throw_to_monkey_nb
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Monkey {
    nb_inspected_items: usize,
    starting_items: Vec<u64>,
    operation: Operation,
    test: Test,
}

fn compute(content: String) -> String {
    let mut monkeys: Vec<Monkey> = Default::default();
    for monkey_description in content.split("\n\n") {
        let mut monkey: Monkey = Default::default();
        let (_, rest) = monkey_description.split_once('\n').unwrap();

        let (starting_items, rest) = rest.split_once('\n').unwrap();
        let (_, starting_items) = starting_items.split_once(": ").unwrap();
        monkey.starting_items = starting_items
            .split(", ")
            .map(|item| item.parse::<u64>().unwrap())
            .collect();

        let (operation, rest) = rest.split_once('\n').unwrap();
        let (_, operation) = operation.split_once("= ").unwrap();
        let (first_operand, operation_rest) = operation.split_once(' ').unwrap();
        let (operator, second_operand) = operation_rest.split_once(' ').unwrap();
        monkey.operation = Operation {
            first_operand: Operand::new(first_operand),
            operator: Operator::new(operator),
            second_operand: Operand::new(second_operand),
        };

        let (divisible_by, rest) = rest.split_once('\n').unwrap();
        monkey.test.divisible_by = divisible_by.split(' ').last().unwrap().parse().unwrap();

        let (if_true_throw_to_monkey_nb, rest) = rest.split_once('\n').unwrap();
        monkey.test.if_true_throw_to_monkey_nb = if_true_throw_to_monkey_nb
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();

        monkey.test.if_false_throw_to_monkey_nb =
            rest.split(' ').last().unwrap().trim_end().parse().unwrap();

        monkeys.push(monkey);
    }

    let lcm = monkeys
        .iter()
        .fold(1, |acc, monkey| acc.lcm(&monkey.test.divisible_by));

    for _ in 0..10000 {
        for monkey_index in 0..monkeys.len() {
            monkeys[monkey_index].nb_inspected_items += monkeys[monkey_index].starting_items.len();
            for item in monkeys[monkey_index].starting_items.clone() {
                let mut worry_level = monkeys[monkey_index].operation.calculate(item);
                worry_level %= lcm;
                let monkey_to_throw = monkeys[monkey_index]
                    .test
                    .calculate_which_monkey_to_throw(worry_level);
                monkeys[monkey_to_throw].starting_items.push(worry_level);
            }
            monkeys[monkey_index].starting_items.clear();
        }
    }

    BinaryHeap::from(monkeys)
        .iter()
        .take(2)
        .map(|monkey| monkey.nb_inspected_items)
        .product::<usize>()
        .to_string()
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
Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";
        let expected = "2713310158";
        assert_eq!(compute(content.to_string()), expected);
    }
}
