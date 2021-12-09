use std::io::Error as ioError;

fn main() {
    let data = read_and_process_input().unwrap();

    let part_one_answer = part_one_solution(&data);
    println!("Part One Solution is: {}", part_one_answer);

    let part_two_answer = part_two_solution(&data);
    println!("Part Two Solution is : {}", part_two_answer)
}

fn part_one_solution(data: &Vec<String>) -> isize {
    let total_lines: u32 = data.len().try_into().unwrap();
    let mut gamma: Vec<&str> = vec![];
    // assuming that each number is the same size
    let upper_bound = data.first().unwrap().len();

    for n in 0..upper_bound {
        let total: u32 = data
            .into_iter()
            .map(|num| {
                let digits: Vec<char> = num.chars().collect();
                let iteration_digit = digits[n].to_digit(10);
                return iteration_digit.unwrap();
            })
            .sum();

        let value = if total >= total_lines / 2 { "1" } else { "0" };

        gamma.push(value)
    }

    // TODO: look into whether there's a simpler way to invert a binary (I'm assuming there is)
    let epsilon: Vec<&str> = gamma
        .clone()
        .into_iter()
        .map(|val| if val == "0" { "1" } else { "0" })
        .collect();

    let epsilon_digit = isize::from_str_radix(epsilon.join("").as_str(), 2).unwrap();
    let gamma_digit = isize::from_str_radix(gamma.join("").as_str(), 2).unwrap();
    gamma_digit * epsilon_digit
}

fn part_two_solution(data: &Vec<String>) -> isize {
    let oxygen_generator_string = determine_life_support_rating(data, 0, |ones, zeroes| {
        if ones.len() >= zeroes.len() {
            return ones;
        } else {
            return zeroes;
        }
    });

    let co2_scrubber_string = determine_life_support_rating(data, 0, |ones, zeroes| {
        if zeroes.len() <= ones.len() {
            return zeroes;
        } else {
            return ones;
        }
    });

    let oxygen_generator_rating =
        isize::from_str_radix(oxygen_generator_string.as_str(), 2).unwrap();
    let co2_scrubber_rating = isize::from_str_radix(co2_scrubber_string.as_str(), 2).unwrap();

    oxygen_generator_rating * co2_scrubber_rating
}

fn determine_life_support_rating(
    data: &Vec<String>,
    search_index: usize,
    data_filter: impl Fn(Vec<String>, Vec<String>) -> Vec<String>,
) -> String {
    match data.len() {
        0 => panic!("No data entries found, something went wrong"),
        1 => return data.first().unwrap().to_string(),
        _ => {
            let mut ones: Vec<String> = vec![];
            let mut zeroes: Vec<String> = vec![];

            // TODO: remove unnecessary tuple intermediate value
            for binary_tuple in data.iter().enumerate() {
                let binary_string = binary_tuple.1;
                let chars: Vec<char> = binary_string.chars().collect();
                // assuming that we will always find a definite string before finishing the entire value
                let char = chars[search_index];

                if char == '1' {
                    ones.push(binary_string.to_string());
                } else {
                    zeroes.push(binary_string.to_string());
                }
            }

            let filtered_data = data_filter(ones, zeroes);

            return determine_life_support_rating(&filtered_data, search_index + 1, data_filter);
        }
    }
}

fn read_and_process_input() -> Result<Vec<String>, ioError> {
    let contents = include_str!("../data/puzzle_data");
    Ok(contents.split('\n').map(|val| String::from(val)).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<String> {
        let data = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        data.into_iter().map(|x| String::from(x)).collect()
    }

    #[test]
    fn test_part_one_example() {
        assert_eq!(part_one_solution(&test_data()), 198);
    }

    #[test]
    fn test_part_one_solution() {
        let data = read_and_process_input().unwrap();

        assert_eq!(part_one_solution(&data), 3633500);
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(part_two_solution(&test_data()), 230);
    }

    #[test]
    fn test_part_two_solution() {
        let data = read_and_process_input().unwrap();

        assert_eq!(part_two_solution(&data), 4550283);
    }
}
