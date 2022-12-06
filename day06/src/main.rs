use day06::load;
use day06::puzzle_a;
use day06::puzzle_b;

fn main() {
    let filename = "input";
    let buffer = load(filename);

    let value = puzzle_a(&buffer);
    println!("Answer to 1st question: {}", value);

    let value_b = puzzle_b(&buffer);
    println!("Answer to 2nd question: {}", value_b);
}
