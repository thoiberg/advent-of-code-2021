use std::collections::HashMap;

fn main() {
    let initial_state = process_input(read_input());

    println!(
        "Part One Solution is: {}",
        calculate_fish_population(initial_state, 80),
    );

    let initial_state = process_input(read_input());
    println!(
        "Part Two Solution is: {}",
        calculate_fish_population(initial_state, 256),
    );
}

fn calculate_fish_population(initial_state: Vec<u32>, days: u32) -> u64 {
    fn tick(initial_state: HashMap<u32, u64>, days: u32) -> HashMap<u32, u64> {
        if days == 0 {
            return initial_state;
        }
        let zeroth_day = initial_state.get(&0).unwrap().clone();
        let mut state_after_day: HashMap<u32, u64> = HashMap::new();

        for day in 1..=8 {
            let value = initial_state.get(&day).unwrap().clone();

            state_after_day.insert(day - 1, value);
        }

        let sixth_day = state_after_day.get(&6).unwrap().clone();

        state_after_day.insert(6, sixth_day + zeroth_day);
        state_after_day.insert(8, zeroth_day);

        tick(state_after_day, days - 1)
    }

    let mut initial_map: HashMap<u32, u64> = HashMap::new();

    for day in 0..=8 {
        let sum =
            initial_state
                .clone()
                .into_iter()
                .fold(0 as u64, |acc, x| if x == day { acc + 1 } else { acc });

        initial_map.insert(day, sum);
    }

    let final_state = tick(initial_map, days);

    final_state.values().sum::<u64>()
}

fn process_input(contents: &'static str) -> Vec<u32> {
    contents
        .split(",")
        .into_iter()
        .map(|timer| timer.parse::<u32>().unwrap())
        .collect()
}

fn read_input() -> &'static str {
    include_str!("../data/puzzle_input.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    fn example_data() -> Vec<u32> {
        process_input(include_str!("../data/example_input.txt"))
    }

    #[test]
    fn test_part_one_example() {
        assert_eq!(part_one_solution(example_data(), 80), 5934);
    }

    #[test]
    fn test_part_one_solution() {
        assert_eq!(part_one_solution(process_input(read_input()), 80), 360268);
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(calculate_fish_population(example_data(), 256), 26984457539);
    }

    #[test]
    fn test_part_two_solution() {
        assert_eq!(
            calculate_fish_population(process_input(read_input()), 256),
            1632146183902
        );
    }
}
