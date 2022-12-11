extern crate filelib;

pub use filelib::load_no_blanks;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
enum OperationOperand {
    Constant(u64),
    Old,
}

#[derive(Debug, Clone)]
enum OperationOp {
    Multiply,
    Add,
}

struct Monkey {
    item_worries: Vec<u64>,
    operation_left: OperationOperand,
    operation_right: OperationOperand,
    operation_op: OperationOp,
    test_divisible_by: u64,
    true_target: Option<Rc<RefCell<Monkey>>>,
    false_target: Option<Rc<RefCell<Monkey>>>,
    num_inspected: usize,
    max_product: u64,
}

impl Monkey {
    fn new(
        item_worries: Vec<u64>,
        operation_left: OperationOperand,
        operation_right: OperationOperand,
        test_divisible_by: u64,
        operation_op: OperationOp,
    ) -> Monkey {
        return Monkey {
            item_worries: item_worries,
            operation_left: operation_left,
            operation_right: operation_right,
            operation_op: operation_op,
            test_divisible_by: test_divisible_by,
            true_target: None,
            false_target: None,
            num_inspected: 0,
            max_product: 0,
        };
    }

    fn compute_worries(&mut self, divide_by_three: bool) {
        for index in 0..self.item_worries.len() {
            let old = self.item_worries[index];
            let left = match self.operation_left {
                OperationOperand::Constant(v) => v,
                OperationOperand::Old => old,
            };
            let right = match self.operation_right {
                OperationOperand::Constant(v) => v,
                OperationOperand::Old => old,
            };
            let inspect_value = match self.operation_op {
                OperationOp::Multiply => left * right,
                OperationOp::Add => left + right,
            };
            let new;
            if divide_by_three {
                new = inspect_value / 3;
            } else {
                new = inspect_value % self.max_product;
            }
            self.item_worries[index] = new;
            self.num_inspected += 1;
        }
    }

    fn give_item(&mut self, worry: u64) {
        self.item_worries.push(worry);
    }

    fn test_items(&mut self) {
        let true_target = match &self.true_target {
            Some(v) => v.clone(),
            None => panic!("Failed to set items"),
        };
        let false_target = match &self.false_target {
            Some(v) => v.clone(),
            None => panic!("Failed to set items"),
        };

        for item in &self.item_worries {
            if item % self.test_divisible_by == 0 {
                true_target.borrow_mut().give_item(*item);
            } else {
                false_target.borrow_mut().give_item(*item);
            }
        }
        self.item_worries = vec![];
    }
}

pub fn produce_sample_input() -> Vec<String> {
    return vec![
        "Monkey 0:",
        "  Starting items: 79, 98",
        "  Operation: new = old * 19",
        "  Test: divisible by 23",
        "    If true: throw to monkey 2",
        "    If false: throw to monkey 3",
        "Monkey 1:",
        "  Starting items: 54, 65, 75, 74",
        "  Operation: new = old + 6",
        "  Test: divisible by 19",
        "    If true: throw to monkey 2",
        "    If false: throw to monkey 0",
        "Monkey 2:",
        "  Starting items: 79, 60, 97",
        "  Operation: new = old * old",
        "  Test: divisible by 13",
        "    If true: throw to monkey 1",
        "    If false: throw to monkey 3",
        "Monkey 3:",
        "  Starting items: 74",
        "  Operation: new = old + 3",
        "  Test: divisible by 17",
        "    If true: throw to monkey 0",
        "    If false: throw to monkey 1",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn parse_input(input: &Vec<String>) -> Vec<Rc<RefCell<Monkey>>> {
    let mut monkeys: Vec<Rc<RefCell<Monkey>>> = vec![];
    let mut monkey_trues: Vec<usize> = vec![];
    let mut monkey_falses: Vec<usize> = vec![];
    let mut monkey_tests_product: u64 = 1;

    // Create all the monkeys first, then handle the true and falses
    let mut temp_worries = vec![];
    let mut temp_op_left = OperationOperand::Old;
    let mut temp_op_right = OperationOperand::Old;
    let mut temp_op = OperationOp::Multiply;
    let mut temp_test = 1;

    for line in input {
        if line.starts_with("Monkey") && !line.starts_with("Monkey 0") {
            // This monkey is done
            monkeys.push(Rc::new(RefCell::new(Monkey::new(
                temp_worries.clone(),
                temp_op_left.clone(),
                temp_op_right.clone(),
                temp_test.clone(),
                temp_op.clone(),
            ))));
            monkey_tests_product *= temp_test;
        } else if line.trim().starts_with("Starting") {
            let (_, all_worries) = line.split_once(": ").unwrap();
            temp_worries = all_worries
                .split(", ")
                .map(|s| s.parse::<u64>().unwrap())
                .collect();
        } else if line.trim().starts_with("Operation") {
            let (_, equation) = line.split_once("new = ").unwrap();
            let equation_parts: Vec<&str> = equation.split(" ").collect();
            if equation_parts[0] == "old" {
                temp_op_left = OperationOperand::Old;
            } else {
                temp_op_left =
                    OperationOperand::Constant(equation_parts[0].parse::<u64>().unwrap());
            }
            if equation_parts[1] == "*" {
                temp_op = OperationOp::Multiply;
            } else {
                temp_op = OperationOp::Add;
            }
            if equation_parts[2] == "old" {
                temp_op_right = OperationOperand::Old;
            } else {
                temp_op_right =
                    OperationOperand::Constant(equation_parts[2].parse::<u64>().unwrap());
            }
        } else if line.trim().starts_with("Test") {
            let (_, v) = line.split_once("by ").unwrap();
            temp_test = v.parse::<u64>().unwrap();
        } else if line.trim().starts_with("If true") {
            let (_, v) = line.split_once("monkey ").unwrap();
            monkey_trues.push(v.parse::<usize>().unwrap());
        } else if line.trim().starts_with("If false") {
            let (_, v) = line.split_once("monkey ").unwrap();
            monkey_falses.push(v.parse::<usize>().unwrap());
        }
    }
    // Should have one monkey unfinished
    monkeys.push(Rc::new(RefCell::new(Monkey::new(
        temp_worries.clone(),
        temp_op_left.clone(),
        temp_op_right.clone(),
        temp_test.clone(),
        temp_op.clone(),
    ))));
    monkey_tests_product *= temp_test;

    // Now assign targets
    for i in 0..monkeys.len() {
        monkeys[i].borrow_mut().true_target = Some(monkeys[monkey_trues[i]].clone());
        monkeys[i].borrow_mut().false_target = Some(monkeys[monkey_falses[i]].clone());
        monkeys[i].borrow_mut().max_product = monkey_tests_product;
    }

    return monkeys;
}

/// Solution to puzzle_a entry point
/// ```
/// let vec1: Vec<String> = day11::produce_sample_input();
/// assert_eq!(day11::puzzle_a(&vec1), 10605);
/// ```
pub fn puzzle_a(input: &Vec<String>) -> usize {
    let rounds = 20;
    let monkeys: Vec<Rc<RefCell<Monkey>>> = parse_input(input);
    for _ in 0..rounds {
        for index in 0..monkeys.len() {
            let monkey = monkeys[index].clone();
            monkey.borrow_mut().compute_worries(true);
            monkey.borrow_mut().test_items();
        }
    }
    let mut inspected: Vec<usize> = monkeys.iter().map(|m| m.borrow().num_inspected).collect();
    inspected.sort();
    return inspected.pop().unwrap() * inspected.pop().unwrap();
}

/// Solution to puzzle_b entry point
/// ```
/// let vec1: Vec<String> = day11::produce_sample_input();
/// assert_eq!(day11::puzzle_b(&vec1), 2713310158);
/// ```
pub fn puzzle_b(input: &Vec<String>) -> usize {
    let rounds = 10000;
    let monkeys: Vec<Rc<RefCell<Monkey>>> = parse_input(input);
    for _ in 0..rounds {
        for index in 0..monkeys.len() {
            let monkey = monkeys[index].clone();
            monkey.borrow_mut().compute_worries(false);
            monkey.borrow_mut().test_items();
        }
    }
    let mut inspected: Vec<usize> = monkeys.iter().map(|m| m.borrow().num_inspected).collect();
    inspected.sort();
    return inspected.pop().unwrap() * inspected.pop().unwrap();
}
