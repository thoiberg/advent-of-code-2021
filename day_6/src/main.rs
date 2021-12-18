fn main() {
    let initial_state = process_input(read_input());

    println!(
        "Part One Solution is: {}",
        part_one_solution(initial_state, 80),
    );
}

fn part_one_solution(initial_state: Vec<u32>, days: u32) -> usize {
    if days == 0 {
        initial_state.len()
    } else {
        let mut new_fish: Vec<u32> = vec![];

        let mut next_state: Vec<u32> = initial_state
            .into_iter()
            .map(|timer| match timer {
                0 => {
                    new_fish.push(8);
                    6
                }
                _ => timer - 1,
            })
            .collect();

        next_state.append(&mut new_fish);

        part_one_solution(next_state, days - 1)
    }
    // if days is 0 then return the count of the array
    // map through the array decrement the values by one
    // if the value is 0 then return 6 and append an 8 to the array
    //    (might have to put it into a separate array, and append that to the end once the loop is done)
    // call return part_one_solution again with the new array and decrement the day counter
    // 0
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
}
