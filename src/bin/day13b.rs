use std::cmp::Ordering;
use std::fmt;
use std::fs;
use std::ops;

#[derive(Debug, Clone, Eq)]
enum Packet {
    List(Vec<Packet>),
    Integer(u32),
    Empty,
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ops::Index<usize> for Packet {
    type Output = Packet;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Packet::List(list) => &list[index],
            _ => panic!("Packet is not List"),
        }
    }
}

impl ops::IndexMut<usize> for Packet {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            Packet::List(list) => &mut list[index],
            _ => panic!("Packet is not List"),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match is_pair_ordered(self, other) {
            None => Ordering::Equal,
            Some(true) => Ordering::Less,
            Some(false) => Ordering::Greater,
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        is_pair_ordered(self, other).is_none()
    }
}

fn get_list_from_packet(packet: &mut Packet) -> &mut Vec<Packet> {
    match packet {
        Packet::List(current_list) => current_list,
        _ => panic!("failed to get list from packet"),
    }
}

fn get_sub_packet<'a>(packet: &'a mut Packet, indexes: &[usize]) -> &'a mut Packet {
    indexes.iter().fold(packet, |acc, index| &mut acc[*index])
}

fn parse_packet(packet_to_parse: &str) -> Packet {
    let mut packet = Packet::Empty;
    let mut integer = String::new();
    let mut indexes = vec![];
    for c in packet_to_parse.chars() {
        match c {
            '[' => {
                if matches!(packet, Packet::Empty) {
                    packet = Packet::List(vec![]);
                } else {
                    let current_packet = get_sub_packet(&mut packet, &indexes);
                    let list = get_list_from_packet(current_packet);
                    indexes.push(list.len());
                    list.push(Packet::List(vec![]));
                }
            }
            ']' | ',' => {
                let current_packet = get_sub_packet(&mut packet, &indexes);
                if !integer.is_empty() {
                    let list = get_list_from_packet(current_packet);
                    list.push(Packet::Integer(integer.parse().unwrap()));
                }
                integer.clear();

                if c == ']' {
                    indexes.pop();
                }
            }
            c => integer.push(c),
        }
    }
    packet
}

fn is_pair_ordered(left: &Packet, right: &Packet) -> Option<bool> {
    match (left, right) {
        (Packet::Integer(left), Packet::Integer(right)) => match left.cmp(right) {
            Ordering::Equal => None,
            Ordering::Less => Some(true),
            Ordering::Greater => Some(false),
        },
        (Packet::List(left), Packet::List(right)) => {
            let mut is_ordered = None;
            for (left, right) in left.iter().zip(right) {
                is_ordered = is_pair_ordered(left, right);
                if is_ordered.is_some() {
                    break;
                }
            }
            if is_ordered.is_none() {
                is_ordered = match left.len().cmp(&right.len()) {
                    Ordering::Equal => None,
                    Ordering::Less => Some(true),
                    Ordering::Greater => Some(false),
                }
            }
            is_ordered
        }
        (left, right) if matches!((left, right), (Packet::List(_), Packet::Integer(_))) => {
            is_pair_ordered(left, &Packet::List(vec![right.clone()]))
        }
        (left, right) if matches!((left, right), (Packet::Integer(_), Packet::List(_))) => {
            is_pair_ordered(&Packet::List(vec![left.clone()]), right)
        }
        _ => panic!(),
    }
}

fn compute(content: String) -> String {
    let first_divider = parse_packet("[[2]]");
    let second_divider = parse_packet("[[6]]");
    let mut packets: Vec<Packet> = vec![first_divider.clone(), second_divider.clone()];
    content
        .trim_end()
        .split("\n\n")
        .map(|pair| pair.split_once('\n').unwrap())
        .for_each(|(left_packet, right_packet)| {
            packets.extend([parse_packet(left_packet), parse_packet(right_packet)])
        });

    packets.sort();
    let first_divider_index = packets
        .iter()
        .position(|packet| packet == &first_divider)
        .unwrap()
        + 1;
    let second_divider_index = packets
        .iter()
        .position(|packet| packet == &second_divider)
        .unwrap()
        + 1;
    let decoder_key = first_divider_index * second_divider_index;
    decoder_key.to_string()
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
    fn test_parse_packet() {
        assert_eq!(
            parse_packet("[]").to_string(),
            Packet::List(vec![]).to_string()
        );
        assert_eq!(
            parse_packet("[[]]").to_string(),
            Packet::List(vec![Packet::List(vec![])]).to_string(),
        );
        assert_eq!(
            parse_packet("[[[]]]").to_string(),
            Packet::List(vec![Packet::List(vec![Packet::List(vec![])])]).to_string(),
        );
        assert_eq!(
            parse_packet("[1,1,3,1,1]").to_string(),
            Packet::List(vec![
                Packet::Integer(1),
                Packet::Integer(1),
                Packet::Integer(3),
                Packet::Integer(1),
                Packet::Integer(1),
            ])
            .to_string(),
        );
        assert_eq!(
            parse_packet("[[1],4]").to_string(),
            Packet::List(vec![
                Packet::List(vec![Packet::Integer(1)]),
                Packet::Integer(4),
            ])
            .to_string(),
        );
        assert_eq!(
            parse_packet("[[1],[2,3,4]]").to_string(),
            Packet::List(vec![
                Packet::List(vec![Packet::Integer(1)]),
                Packet::List(vec![
                    Packet::Integer(2),
                    Packet::Integer(3),
                    Packet::Integer(4),
                ]),
            ])
            .to_string(),
        );
        assert_eq!(
            parse_packet("[1,[2,[3,[4,[5,6,7]]]],8,9]").to_string(),
            Packet::List(vec![
                Packet::Integer(1),
                Packet::List(vec![
                    Packet::Integer(2),
                    Packet::List(vec![
                        Packet::Integer(3),
                        Packet::List(vec![
                            Packet::Integer(4),
                            Packet::List(vec![
                                Packet::Integer(5),
                                Packet::Integer(6),
                                Packet::Integer(7),
                            ]),
                        ]),
                    ]),
                ]),
                Packet::Integer(8),
                Packet::Integer(9),
            ])
            .to_string(),
        );
    }

    #[test]
    fn test_is_pair_ordered() {
        assert_eq!(
            is_pair_ordered(&parse_packet("[1,1,3,1,1]"), &parse_packet("[1,1,5,1,1]")),
            Some(true),
        );
        assert_eq!(
            is_pair_ordered(&parse_packet("[[1],[2,3,4]]"), &parse_packet("[[1],4]")),
            Some(true),
        );
        assert_eq!(
            is_pair_ordered(&parse_packet("[9]"), &parse_packet("[[8,7,6]]")),
            Some(false),
        );
        assert_eq!(
            is_pair_ordered(&parse_packet("[[4,4],4,4]"), &parse_packet("[[4,4],4,4,4]")),
            Some(true),
        );
        assert_eq!(
            is_pair_ordered(&parse_packet("[7,7,7,7]"), &parse_packet("[7,7,7]")),
            Some(false),
        );
        assert_eq!(
            is_pair_ordered(&parse_packet("[]"), &parse_packet("[3]")),
            Some(true),
        );
        assert_eq!(
            is_pair_ordered(&parse_packet("[[[]]]"), &parse_packet("[[]]")),
            Some(false),
        );
        assert_eq!(
            is_pair_ordered(
                &parse_packet("[1,[2,[3,[4,[5,6,7]]]],8,9]"),
                &parse_packet("[1,[2,[3,[4,[5,6,0]]]],8,9]")
            ),
            Some(false),
        );
    }

    #[test]
    fn examples() {
        let content = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";
        let expected = "140";
        assert_eq!(compute(content.to_string()), expected);
    }
}
