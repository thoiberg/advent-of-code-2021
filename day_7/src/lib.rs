pub fn part_one_solution(starting_positions: Vec<u32>) -> i32 {
    let mut lowest_fuel_cost = i32::MAX;

    for i in &starting_positions {
        let current_cost = starting_positions.clone().into_iter().fold(0, |acc, j| {
            let new_val = ((*i as i32) - (j as i32)).abs();
            acc + new_val
        });

        if current_cost < lowest_fuel_cost {
            lowest_fuel_cost = current_cost;
        }
    }

    lowest_fuel_cost
}

pub fn part_two_solution(starting_positions: Vec<u32>) -> i32 {
    let mut lowest_fuel_cost = i32::MAX;

    // find the max val in starting positions and iterate between 0 and that
    let max = starting_positions.iter().max().unwrap().clone();

    for i in 0..=max {
        let current_cost = starting_positions.clone().into_iter().fold(0, |acc, j| {
            let distance = ((i as i32) - (j as i32)).abs();

            let cost: i32 = (1..=distance).sum();
            acc + cost
        });

        if current_cost < lowest_fuel_cost {
            lowest_fuel_cost = current_cost;
        }
    }

    lowest_fuel_cost
}

pub fn process_input() -> Vec<u32> {
    include_str!("../data/puzzle_data.txt")
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<u32> {
        vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]
    }

    #[test]
    fn test_part_one_example() {
        assert_eq!(part_one_solution(test_data()), 37);
    }

    #[test]
    fn test_part_one_solution() {
        assert_eq!(part_one_solution(process_input()), 343468);
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(part_two_solution(test_data()), 168);
    }

    #[test]
    fn test_part_two_solution() {
        assert_eq!(part_two_solution(process_input()), 96086265);
    }
}
