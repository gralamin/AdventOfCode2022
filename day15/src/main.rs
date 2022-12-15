use day15::load_no_blanks;
use day15::puzzle_a;
use day15::puzzle_b;

fn main() {
    let filename = "input";
    let template = load_no_blanks(filename);
    let answer_row = 2000000;

    let value = puzzle_a(&template, answer_row);
    println!("Answer to 1st question: {}", value);

    let max_coord = 4000000;
    let value_b = puzzle_b(&template, max_coord);
    println!("Answer to 2nd question: {}", value_b);
}
