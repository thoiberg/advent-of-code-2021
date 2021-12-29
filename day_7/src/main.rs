mod lib;

use crate::lib::*;

fn main() {
    println!(
        "Part One Solution is: {}",
        part_one_solution(process_input())
    );

    println!(
        "Part Two Solution is: {}",
        part_two_solution(process_input())
    );
}
