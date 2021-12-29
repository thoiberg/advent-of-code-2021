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

pub fn process_input() -> Vec<u32> {
    include_str!("../data/puzzle_data.txt")
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let data = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        assert_eq!(part_one_solution(data), 37);
    }
}
