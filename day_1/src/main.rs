use std::io::Error as ioError;

fn main() {
    let data = read_and_process_input().unwrap();

    let first_solution = part_one_solution(&data);
    let second_solution = part_two_solution(&data);

    println!("Part One Solution is: {}", first_solution);
    println!("Part Two Solution is: {}", second_solution);
}

fn part_one_solution(data: &Vec<i32>) -> i32 {
    check_depth_increased(data.first().unwrap(), &data[1..].to_vec(), 0)
}

fn check_depth_increased(previous: &i32, data: &Vec<i32>, count: i32) -> i32 {
    let current = data.first();

    match current {
        Some(x) => {
            let mut new_count = count;
            if x > previous {
                new_count += 1;
            }

            return check_depth_increased(x, &data[1..].to_vec(), new_count);
        }
        None => return count,
    }
}

fn part_two_solution(data: &Vec<i32>) -> i32 {
    let first = data.get(0);
    let second = data.get(1);

    check_sliding_window(first, second, &data[2..].to_vec(), 0)
}

fn check_sliding_window(
    first_prev: Option<&i32>,
    second_prev: Option<&i32>,
    data: &Vec<i32>,
    count: i32,
) -> i32 {
    let first = first_prev.unwrap_or(&0);
    let second = second_prev.unwrap_or(&0);

    let current = data.first();

    match current {
        Some(x) => {
            let sliding_count = x + first + second;
            let next_sliding_count = second + x + data.get(1).unwrap_or(&0);

            let new_count = if next_sliding_count > sliding_count {
                count + 1
            } else {
                count
            };

            return check_sliding_window(second_prev, current, &data[1..].to_vec(), new_count);
        }
        None => return count,
    }
}

fn read_and_process_input() -> Result<Vec<i32>, ioError> {
    let contents = include_str!("input_data");
    Ok(contents
        .split('\n')
        .map(|val| val.parse::<i32>().unwrap())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let increase_count = part_one_solution(&data);

        assert_eq!(increase_count, 7);
    }

    #[test]
    fn test_part_one_solution() {
        let data = read_and_process_input().unwrap();
        let increase_count = part_one_solution(&data);

        assert_eq!(increase_count, 1583);
    }

    #[test]
    fn test_part_two_example() {
        let data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let increase_count = part_two_solution(&data);

        assert_eq!(increase_count, 5);
    }

    #[test]
    fn test_part_two_solution() {
        let data = read_and_process_input().unwrap();
        let increase_count = part_two_solution(&data);

        assert_eq!(increase_count, 1627);
    }
}
