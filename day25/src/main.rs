use day25::load_no_blanks;
use day25::puzzle_a;

fn main() {
    let filename = "input";
    let template = load_no_blanks(filename);

    let value = puzzle_a(&template);
    println!("Answer to 1st question: {}", value);
}
