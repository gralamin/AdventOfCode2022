use day05::load;
use day05::parse_moves;
use day05::parse_stacks;
use day05::puzzle_a;
use day05::puzzle_b;

fn main() {
    let filename = "input";
    let file_input = load(filename);
    let (stacks_raw, moves_raw) = file_input.split_once("\n\n").unwrap();
    let stacks = parse_stacks(stacks_raw);
    let moves = parse_moves(moves_raw);

    let value = puzzle_a(&stacks, &moves);
    println!("Answer to 1st question: {}", value);

    let value_b = puzzle_b(&stacks, &moves);
    println!("Answer to 2nd question: {}", value_b);
}
