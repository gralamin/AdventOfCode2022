use day02::load_no_blanks;
use day02::puzzle_a;
use day02::puzzle_a_sol2;
use day02::puzzle_b;
use day02::puzzle_b_sol2;

fn main() {
    let filename = "input";
    let rounds = load_no_blanks(filename);

    let value = puzzle_a(&rounds);
    println!("Answer to 1st question: {}", value);

    let value_b = puzzle_b(&rounds);
    println!("Answer to 2nd question: {}", value_b);

    println!("--------------------");
    println!("Mathy solution");
    let value_2 = puzzle_a_sol2(&rounds);
    println!("Answer to 1st question: {}", value_2);

    let value_b_2 = puzzle_b_sol2(&rounds);
    println!("Answer to 2nd question: {}", value_b_2);
}
