use std::collections::HashMap;

mod line;

use line::*;

fn main() {
    let lines = process_input(read_input());

    let part_one_answer = part_one_solution(&lines);
    println!("Part One Solution is: {}", part_one_answer);

    let part_two_answer = part_two_solution(&lines);
    println!("Part Two Solution is: {}", part_two_answer);
}

fn part_one_solution(lines: &Vec<Line>) -> u32 {
    let valid_lines: Vec<&Line> = lines
        .into_iter()
        .filter(|line| line.is_horizontal() || line.is_vertical())
        .collect();

    let mut position_hash: HashMap<String, u32> = HashMap::new();

    for line in valid_lines {
        for point in line.each_point() {
            let id = format!("{}_{}", point.x, point.y);

            match position_hash.get(&id) {
                Some(val) => {
                    position_hash.insert(id, val + 1);
                }
                None => {
                    position_hash.insert(id, 1);
                }
            }
        }
    }

    position_hash
        .into_iter()
        .fold(0, |acc, (_, value)| if value > 1 { acc + 1 } else { acc })
}

fn part_two_solution(lines: &Vec<Line>) -> u32 {
    let mut position_hash: HashMap<String, u32> = HashMap::new();
    let new_lines = lines.to_vec();

    for line in new_lines {
        for point in line {
            let id = format!("{}_{}", point.x, point.y);

            match position_hash.get(&id) {
                Some(val) => {
                    position_hash.insert(id, val + 1);
                }
                None => {
                    position_hash.insert(id, 1);
                }
            }
        }
    }

    position_hash
        .into_iter()
        .fold(0, |acc, (_, value)| if value > 1 { acc + 1 } else { acc })
}

fn read_input() -> &'static str {
    include_str!("../data/puzzle_data.txt")
}

fn process_input(input: &str) -> Vec<Line> {
    input
        .split("\n")
        .map(|line| {
            let points: Vec<&str> = line.split(" -> ").collect();
            Line {
                start: Point::new(points[0]),
                end: Point::new(points[1]),
                at_start: true,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<Line> {
        let input = include_str!("../data/test_data.txt");
        process_input(input)
    }

    #[test]
    fn test_part_one_example() {
        assert_eq!(part_one_solution(&test_data()), 5);
    }

    #[test]
    fn test_part_one_solution() {
        assert_eq!(part_one_solution(&process_input(read_input())), 5147);
    }

    #[test]
    fn test_part_two_solution_example() {
        assert_eq!(part_two_solution(&test_data()), 12);
    }
}
