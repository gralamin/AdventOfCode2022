use day22::load;
use day22::puzzle_a;
use day22::puzzle_b;

fn main() {
    let filename = "input";
    let template = load(filename);

    let value = puzzle_a(&template);
    println!("Answer to 1st question: {}", value);

    let value_b = puzzle_b(&template);
    println!("Answer to 2nd question: {}", value_b);
}
