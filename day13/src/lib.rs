extern crate filelib;

pub use filelib::load;
pub use filelib::split_lines_by_blanks;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub fn example_input() -> Vec<Vec<String>> {
    return vec![
        vec!["[1,1,3,1,1]", "[1,1,5,1,1]"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        vec!["[[1],[2,3,4]]", "[[1],4]"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        vec!["[9]", "[[8,7,6]]"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        vec!["[[4,4],4,4]", "[[4,4],4,4,4]"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        vec!["[7,7,7,7]", "[7,7,7]"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        vec!["[]", "[3]"].iter().map(|s| s.to_string()).collect(),
        vec!["[[[]]]", "[[]]"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        vec!["[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
    ];
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Tokens {
    Value(i32),
    List(Vec<Rc<RefCell<Tokens>>>),
}

impl Ord for Tokens {
    fn cmp(&self, other: &Self) -> Ordering {
        match compare_pair(
            Rc::new(RefCell::new(self.clone())),
            Rc::new(RefCell::new(other.clone())),
        ) {
            CompareBool::True => Ordering::Less,
            CompareBool::False => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}

impl PartialOrd for Tokens {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

fn parse_packets(packets: &Vec<Vec<String>>) -> Vec<(Tokens, Tokens)> {
    let mut result = vec![];
    for packet_pair in packets {
        let mut p1: Tokens = Tokens::Value(99999);
        let mut p2: Tokens = Tokens::Value(99999);
        let mut first = true;
        for packet_line in packet_pair {
            let mut token_stack = vec![];
            let mut cur_string = "".to_string();
            let mut cur_token: Tokens = Tokens::List(vec![]);
            let mut first_char = true;
            for c in packet_line.chars() {
                if first_char {
                    first_char = false;
                    continue;
                }
                if c == '[' {
                    token_stack.push(cur_token);
                    cur_token = Tokens::List(vec![]);
                } else if c == ']' {
                    if cur_string.len() > 0 {
                        //println!("Parsing 1 '{}'", cur_string);
                        let v = cur_string.parse::<i32>().unwrap();
                        cur_string = "".to_string();
                        match cur_token {
                            Tokens::Value(_) => panic!("Shouldn't happen"),
                            Tokens::List(ref mut l) => {
                                l.push(Rc::new(RefCell::new(Tokens::Value(v))))
                            }
                        };
                    }
                    if token_stack.len() > 0 {
                        let mut v = token_stack.pop().unwrap();
                        match v {
                            Tokens::Value(_) => panic!("Shouldn't happen"),
                            Tokens::List(ref mut l) => l.push(Rc::new(RefCell::new(cur_token))),
                        };
                        cur_token = v;
                    }
                } else if c == ',' {
                    if cur_string.len() > 0 {
                        //println!("Parsing 2 '{}'", cur_string);
                        let v = cur_string.parse::<i32>().unwrap();
                        cur_string = "".to_string();
                        match cur_token {
                            Tokens::Value(_) => panic!("Shouldn't happen"),
                            Tokens::List(ref mut l) => {
                                l.push(Rc::new(RefCell::new(Tokens::Value(v))))
                            }
                        };
                    }
                } else {
                    cur_string.push(c);
                }
            }
            if first {
                p1 = cur_token;
                first = false;
            } else {
                p2 = cur_token;
            }
        }
        result.push((p1, p2));
    }
    return result;
}

#[derive(Debug)]
enum CompareBool {
    True,
    False,
    KeepGoing,
}

impl Display for CompareBool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s: &str = match self {
            CompareBool::True => "True",
            CompareBool::False => "False",
            CompareBool::KeepGoing => "KeepGoing",
        };
        return write!(f, "{}", s);
    }
}

fn compare_pair(packet1: Rc<RefCell<Tokens>>, packet2: Rc<RefCell<Tokens>>) -> CompareBool {
    return match &*packet1.borrow() {
        Tokens::Value(i1) => match &*packet2.borrow() {
            Tokens::Value(i2) => compare_int(*i1, *i2),
            Tokens::List(v2) => {
                let v1 = vec![Rc::new(RefCell::new(Tokens::Value(*i1)))];
                return compare_lists(v1, v2.clone());
            }
        },
        Tokens::List(v1) => {
            return match &*packet2.borrow() {
                Tokens::Value(i2) => {
                    let v2 = vec![Rc::new(RefCell::new(Tokens::Value(*i2)))];
                    return compare_lists(v1.clone(), v2);
                }
                Tokens::List(v2) => compare_lists(v1.clone(), v2.clone()),
            };
        }
    };
}

fn compare_int(i1: i32, i2: i32) -> CompareBool {
    //println!("Comparing {}, {}", i1, i2);
    if i1 < i2 {
        return CompareBool::True;
    }
    if i1 > i2 {
        return CompareBool::False;
    }
    return CompareBool::KeepGoing;
}

fn compare_lists(list1: Vec<Rc<RefCell<Tokens>>>, list2: Vec<Rc<RefCell<Tokens>>>) -> CompareBool {
    let length_lists: Vec<usize> = vec![list1.len(), list2.len()];
    let higher_length = *length_lists.iter().max().unwrap();
    //println!("Comparing Vec of len {}, and len {}", list1.len(), list2.len());
    for i in 0..higher_length {
        if i < list2.len() && i < list1.len() {
            let decision = compare_pair(list1[i].clone(), list2[i].clone());
            let should_return = match decision {
                CompareBool::True => true,
                CompareBool::False => true,
                CompareBool::KeepGoing => false,
            };
            if should_return {
                return decision;
            }
        }
        if i >= list2.len() {
            return CompareBool::False;
        }
        if i >= list1.len() {
            return CompareBool::True;
        }
    }
    return CompareBool::KeepGoing;
}

/// Solution to puzzle_a entry point
/// ```
/// let vec1 = day13::example_input();
/// assert_eq!(day13::puzzle_a(&vec1), 13);
/// ```
pub fn puzzle_a(input: &Vec<Vec<String>>) -> usize {
    let packet_pairs = parse_packets(input);
    let mut sum = 0;
    for i in 0..packet_pairs.len() {
        let (p1, p2) = &packet_pairs[i];
        let result = compare_pair(
            Rc::new(RefCell::new(p1.clone())),
            Rc::new(RefCell::new(p2.clone())),
        );
        //println!("i={}, r={}", i, result);
        match result {
            CompareBool::True => sum += i + 1,
            _ => (),
        }
    }
    return sum;
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct OrderWrapper {
    token: Tokens,
}

impl Ord for OrderWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        match compare_pair(
            Rc::new(RefCell::new(self.token.clone())),
            Rc::new(RefCell::new(other.token.clone())),
        ) {
            CompareBool::True => Ordering::Less,
            CompareBool::False => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}

/// Solution to puzzle_b entry point
/// ```
/// let vec1 = day13::example_input();
/// assert_eq!(day13::puzzle_b(&vec1), 140);
/// ```
pub fn puzzle_b(input: &Vec<Vec<String>>) -> usize {
    let mut packet_pairs = parse_packets(input);
    let divider1 = Tokens::List(vec![Rc::new(RefCell::new(Tokens::List(vec![Rc::new(
        RefCell::new(Tokens::Value(2)),
    )])))]);
    let divider2 = Tokens::List(vec![Rc::new(RefCell::new(Tokens::List(vec![Rc::new(
        RefCell::new(Tokens::Value(6)),
    )])))]);
    packet_pairs.push((divider1.clone(), divider2.clone()));
    let mut just_vec: Vec<OrderWrapper> = packet_pairs
        .into_iter()
        .map(|(p1, p2)| vec![OrderWrapper { token: p1 }, OrderWrapper { token: p2 }])
        .flatten()
        .collect();
    just_vec.sort();

    let mut divider_indexes = 1;

    divider_indexes *= just_vec
        .iter()
        .position(|o| {
            *o.clone()
                == OrderWrapper {
                    token: divider1.clone(),
                }
        })
        .unwrap()
        + 1;
    divider_indexes *= just_vec
        .iter()
        .position(|o| {
            *o.clone()
                == OrderWrapper {
                    token: divider2.clone(),
                }
        })
        .unwrap()
        + 1;
    return divider_indexes;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_pair1() {
        let input = vec![vec!["[[1],[2,3,4]]", "[[1],4]"]
            .iter()
            .map(|s| s.to_string())
            .collect()];
        let inner_one = Tokens::Value(1);
        let inner_two = Tokens::Value(2);
        let inner_three = Tokens::Value(3);
        let inner_four = Tokens::Value(4);
        let one_rc = Rc::new(RefCell::new(inner_one));
        let two_rc = Rc::new(RefCell::new(inner_two));
        let three_rc = Rc::new(RefCell::new(inner_three));
        let four_rc = Rc::new(RefCell::new(inner_four));

        let top_one = Tokens::List(vec![one_rc.clone()]);
        let top_one_rc = Rc::new(RefCell::new(top_one));
        let top_two = Tokens::List(vec![two_rc.clone(), three_rc.clone(), four_rc.clone()]);
        let top = Tokens::List(vec![top_one_rc.clone(), Rc::new(RefCell::new(top_two))]);
        let bottom = Tokens::List(vec![top_one_rc.clone(), four_rc.clone()]);
        let expected = vec![(top, bottom)];
        assert_eq!(parse_packets(&input), expected);
    }
}
