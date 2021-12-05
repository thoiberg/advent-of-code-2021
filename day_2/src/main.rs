use std::io::Error as ioError;
use std::str::FromStr;
use strum_macros::EnumString;

fn main() {
    let data = read_and_process_input().unwrap();

    let part_one_answer = part_one_solution(&data);
    println!("Part One Solution is: {}", part_one_answer);

    let part_two_answer = part_two_solution(&data);
    println!("Part Two Solution is: {}", part_two_answer)
}

#[derive(EnumString, Clone)]
enum Direction {
    #[strum(serialize = "forward")]
    Forward,

    #[strum(serialize = "down")]
    Down,

    #[strum(serialize = "up")]
    Up,
}

#[derive(Clone)]
struct Movement {
    direction: Direction,
    distance: i32,
}

#[derive(Default)]
struct Coordinate {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Coordinate {
    pub fn new() -> Self {
        Default::default()
    }
}

fn part_one_solution(data: &Vec<Movement>) -> i32 {
    let start_point = Coordinate::new();
    let final_coordinate =
        pilot_submarine(start_point, data, |coordinate, movement| {
            match movement.direction {
                Direction::Forward => Coordinate {
                    horizontal: coordinate.horizontal + movement.distance,
                    ..coordinate
                },
                Direction::Down => Coordinate {
                    depth: coordinate.depth + movement.distance,
                    ..coordinate
                },
                Direction::Up => Coordinate {
                    depth: coordinate.depth - movement.distance,
                    ..coordinate
                },
            }
        });

    final_coordinate.depth * final_coordinate.horizontal
}

fn part_two_solution(data: &Vec<Movement>) -> i32 {
    let start_point = Coordinate::new();

    let final_coordinate =
        pilot_submarine(start_point, data, |coordinate, movement| {
            match movement.direction {
                Direction::Forward => Coordinate {
                    horizontal: coordinate.horizontal + movement.distance,
                    depth: coordinate.depth + (coordinate.aim * movement.distance),
                    ..coordinate
                },
                Direction::Down => Coordinate {
                    aim: coordinate.aim + movement.distance,
                    ..coordinate
                },
                Direction::Up => Coordinate {
                    aim: coordinate.aim - movement.distance,
                    ..coordinate
                },
            }
        });

    final_coordinate.horizontal * final_coordinate.depth
}

fn pilot_submarine(
    coordinate: Coordinate,
    movements: &Vec<Movement>,
    move_submarine: impl Fn(Coordinate, &Movement) -> Coordinate,
) -> Coordinate {
    let movement = movements.first();

    match movement {
        Some(x) => {
            let next_coordinate = move_submarine(coordinate, x);

            return pilot_submarine(next_coordinate, &movements[1..].to_vec(), move_submarine);
        }
        None => return coordinate,
    }
}

fn read_and_process_input() -> Result<Vec<Movement>, ioError> {
    let contents = include_str!("input_data");
    Ok(contents
        .split('\n')
        .map(|val| convert_to_movement(val))
        .collect())
}

fn convert_to_movement(instruction: &str) -> Movement {
    let instructions: Vec<&str> = instruction.split(' ').collect();
    Movement {
        direction: Direction::from_str(instructions[0]).unwrap(),
        distance: instructions[1].parse::<i32>().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<Movement> {
        let instructions = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];

        instructions
            .into_iter()
            .map(|instruction| convert_to_movement(instruction))
            .collect()
    }

    #[test]
    fn test_part_one_example() {
        let part_one_answer = part_one_solution(&test_data());

        assert_eq!(part_one_answer, 150);
    }

    #[test]
    fn test_part_one_solution() {
        let data = read_and_process_input().unwrap();
        assert_eq!(part_one_solution(&data), 1670340);
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(part_two_solution(&test_data()), 900);
    }

    #[test]
    fn test_part_two_solution() {
        let data = read_and_process_input().unwrap();
        assert_eq!(part_two_solution(&data), 1954293920);
    }
}
