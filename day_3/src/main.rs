use std::io::Error as ioError;

fn main() {
    let data = read_and_process_input().unwrap();

    let part_one_answer = part_one_solution(&data);
    println!("Part One Solution is: {}", part_one_answer);
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
    // multiply together to get the result
    gamma_digit * epsilon_digit
}

fn read_and_process_input() -> Result<Vec<String>, ioError> {
    let contents = include_str!("../data/puzzle_data");
    Ok(contents.split('\n').map(|val| String::from(val)).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let data = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        let test_data = data.into_iter().map(|x| String::from(x)).collect();

        assert_eq!(part_one_solution(&test_data), 198);
    }

    #[test]
    fn test_part_one_solution() {
        let data = read_and_process_input().unwrap();

        assert_eq!(part_one_solution(&data), 3633500);
    }
}
