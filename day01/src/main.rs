use day01::load;
use day01::puzzle_a;
use day01::puzzle_b;
use day01::split_lines_by_blanks;

fn main() {
    let filename = "input";
    let all_in = load(filename);
    let groups = split_lines_by_blanks(&all_in);
    let int_groups: Vec<Vec<i32>> = groups
        .iter()
        .map(|group| group.iter().map(|s| s.parse().unwrap()).collect())
        .collect();

    let value = puzzle_a(&int_groups);
    println!("Answer to 1st question: {}", value);

    let value_b = puzzle_b(&int_groups);
    println!("Answer to 2nd question: {}", value_b);
}
