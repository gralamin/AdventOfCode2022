extern crate filelib;

pub use filelib::load_no_blanks;
use rustc_hash::FxHashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Monkey {
    name: String,
    left_value: Value,
    op: Option<Operation>,
    right_value: Option<Value>,
}

#[derive(Debug, Clone)]
enum Value {
    Discrete(i64),
    MonkeyPointer(String),
}

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
}

impl Operation {
    fn compute(&self, left_value: i64, right_value: i64) -> i64 {
        return match self {
            Operation::Add => left_value + right_value,
            Operation::Subtract => left_value - right_value,
            Operation::Multiply => left_value * right_value,
            Operation::Divide => left_value / right_value,
            Operation::Equal => {
                if left_value == right_value {
                    1
                } else {
                    // We are going to use an approximation instead of solving for now
                    // But we need the difference to approximate.
                    left_value - right_value
                }
            }
        };
    }
}

fn solve_monkeys(monkies: Vec<Monkey>) -> i64 {
    //println!("In monkies");
    let mut known_values: FxHashMap<String, i64> = FxHashMap::default();
    let mut queue: VecDeque<Monkey> = VecDeque::new();
    // pre compute all Discrete Monkies
    for monkey in monkies {
        match monkey.left_value {
            Value::Discrete(v) => {
                //println!("Adding {} {}", monkey.name, v);
                known_values.insert(monkey.name.clone(), v);
            }
            Value::MonkeyPointer(_) => {
                queue.push_back(monkey);
            }
        };
    }

    // println!("queue: {:?}", queue);

    // Now go through the queue, whenever we have an operation we can't do yet, put to the end of the queue
    while let Some(monkey) = queue.pop_front() {
        let monkey_left = monkey.left_value.clone();
        let monkey_right = monkey.right_value.clone();
        let left = match monkey_left {
            Value::MonkeyPointer(l) => l,
            _ => panic!("Error in logic"),
        };
        let l: &str = &left;
        if !known_values.contains_key(l) {
            queue.push_back(monkey.clone());
            continue;
        }
        let right = match monkey_right.unwrap() {
            Value::MonkeyPointer(r) => r,
            _ => panic!("Error in logic"),
        };
        let r: &str = &right;
        if !known_values.contains_key(r) {
            queue.push_back(monkey.clone());
            continue;
        }
        // We know both, solve this one
        let op = match monkey.op {
            Some(o) => o,
            _ => panic!("Error in logic"),
        };
        let left_value = known_values[l];
        let right_value = known_values[r];
        let name = monkey.name.clone();
        let computed = op.compute(left_value, right_value);
        //println!("Adding {} {}", name, computed);
        known_values.insert(name, computed);
    }
    let final_value = "root".to_string();
    return known_values[&final_value];
}

fn parse_input(lines: &Vec<String>, root_monkey_equal: bool) -> Vec<Monkey> {
    return lines
        .iter()
        .map(|line| {
            let (name, partial) = line.split_once(": ").unwrap();
            let rhs;
            let lhs;
            let op;
            if partial.contains(" ") {
                let (lhs_pointer, rest_equation) = partial.split_once(" ").unwrap();
                let (op_string, rhs_pointer) = rest_equation.split_once(" ").unwrap();
                if name == "root" && root_monkey_equal {
                    op = Some(Operation::Equal);
                } else {
                    op = match op_string {
                        "+" => Some(Operation::Add),
                        "-" => Some(Operation::Subtract),
                        "*" => Some(Operation::Multiply),
                        "/" => Some(Operation::Divide),
                        _ => panic!("unknown op string"),
                    };
                }
                lhs = Value::MonkeyPointer(lhs_pointer.to_string());
                rhs = Some(Value::MonkeyPointer(rhs_pointer.to_string()));
            } else {
                lhs = Value::Discrete(partial.parse::<i64>().unwrap());
                op = None;
                rhs = None;
            }
            return Monkey {
                name: name.to_string(),
                left_value: lhs,
                op: op,
                right_value: rhs,
            };
        })
        .collect();
}

/// Solution to puzzle_a entry point
/// ```
/// let vec1: Vec<String> = vec!["root: pppw + sjmn", "dbpl: 5", "cczh: sllz + lgvd",
///   "zczc: 2", "ptdq: humn - dvpt", "dvpt: 3", "lfqf: 4", "humn: 5", "ljgn: 2",
///   "sjmn: drzm * dbpl", "sllz: 4", "pppw: cczh / lfqf", "lgvd: ljgn * ptdq", "drzm: hmdt - zczc",
///   "hmdt: 32"].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day21::puzzle_a(&vec1), 152);
/// ```
pub fn puzzle_a(input: &Vec<String>) -> i64 {
    let monkey_list = parse_input(input, false);
    //println!("input parsed");
    return solve_monkeys(monkey_list);
}

/// Solution to puzzle_b entry point
/// ```
/// let vec1: Vec<String> = vec!["root: pppw + sjmn", "dbpl: 5", "cczh: sllz + lgvd",
///   "zczc: 2", "ptdq: humn - dvpt", "dvpt: 3", "lfqf: 4", "humn: 5", "ljgn: 2",
///   "sjmn: drzm * dbpl", "sllz: 4", "pppw: cczh / lfqf", "lgvd: ljgn * ptdq", "drzm: hmdt - zczc",
///   "hmdt: 32"].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day21::puzzle_b(&vec1), 301);
/// ```
pub fn puzzle_b(input: &Vec<String>) -> i64 {
    let mut monkey_list = parse_input(input, true);
    let pos = monkey_list.iter().position(|m| m.name == "humn").unwrap();
    let mut human_monkey = monkey_list.remove(pos);
    let mut counter = match human_monkey.left_value.clone() {
        Value::Discrete(n) => n,
        _ => panic!("whoops"),
    };
    loop {
        let cur_monkey = human_monkey.clone();
        let mut cur_list = monkey_list.clone();
        cur_list.push(cur_monkey);
        let result = solve_monkeys(cur_list);
        if result == 1 {
            break;
        } else {
            // approximation algorithm, move massively toward the result, if we are below 100, add to it
            // This will rapidly get us in the correct range, but it isn't a true solution.
            if result < 100 {
                counter += 1;
            } else {
                counter += (result / 100);
            }
        }
        human_monkey.left_value = Value::Discrete(counter);
    }
    return counter;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(1, 1);
    }
}
