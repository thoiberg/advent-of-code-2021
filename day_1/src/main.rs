use std::io::Error as ioError;

fn main() {
    println!("Hello, world!");

    // read the data into an vec of numbers
    let data = read_and_process_input().unwrap();

    let first_solution = part_one_solution(&data);

    println!("Part One Solution is: {}", first_solution);
}

fn part_one_solution(data: &Vec<i32>) -> i32 {
    // recursive method that takes a count var, the current var, and the previous one
    // if current is higher than the previous, increase the count and call the next iteration:
    // current var becomes previous, count is passed in as is, current becomes the next value
    // return the value once the current var is nothing
    check_depth(None, &data, 0)
}

fn check_depth(previous: Option<&i32>, data: &Vec<i32>, count: i32) -> i32 {
    if previous.is_some() {
        let current = data.first();

        match current {
            Some(x) => {
                let mut new_count = count;
                if x > previous.unwrap() {
                    new_count += 1;
                }

                return check_depth(Some(x), &data[1..].to_vec(), new_count);
            }
            None => return count,
        }
    } else {
        return check_depth(data.first(), &data[1..].to_vec(), count);
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
    fn test_part_one_solution() {
        let data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let decrease_count = part_one_solution(&data);

        assert_eq!(decrease_count, 7);
    }
}
