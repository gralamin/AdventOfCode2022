use day15::load_no_blanks;
use day15::puzzle_a;
use day15::puzzle_b;

fn main() {
    let filename = "input";
    let template = load_no_blanks(filename);

    let value = puzzle_a(&template);
    println!("Answer to 1st question: {}", value);

    let value_b = puzzle_b(&template);
    println!("Answer to 2nd question: {}", value_b);
}
