use day04::load_no_blanks;
use day04::parse_line;
use day04::puzzle_a;
use day04::puzzle_b;

fn main() {
    let filename = "input";
    let pairs_str = load_no_blanks(filename);
    let pairs: Vec<(u32, u32, u32, u32)> = pairs_str.iter().map(|s| parse_line(s)).collect();

    let value = puzzle_a(&pairs);
    println!("Answer to 1st question: {}", value);

    let value_b = puzzle_b(&pairs);
    println!("Answer to 2nd question: {}", value_b);
}
