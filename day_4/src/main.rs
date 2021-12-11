mod bingo_board;

use bingo_board::*;

fn main() {
    let data = read_and_process_input();

    let part_one_answer = part_one_solution(&data.0, &data.1);

    println!("Part One Solution is: {}", part_one_answer);

    let part_two_answer = part_two_solution(&data.0, &data.1);

    println!("Part Two Solution is: {}", part_two_answer);
}

fn part_one_solution(bingo_calls: &Vec<i32>, boards: &Vec<BingoBoard>) -> i32 {
    fn process_call(bingo_calls: &Vec<i32>, boards: &Vec<BingoBoard>) -> (BingoBoard, i32) {
        let call = bingo_calls.first().unwrap().to_owned();
        let mut mut_boards = boards.to_owned();

        for board in &mut mut_boards {
            let winner = board.mark(call);

            if winner.to_owned() {
                return (board.to_owned(), call);
            }
        }

        return process_call(&bingo_calls[1..].to_vec(), &mut_boards);
    }

    let (winning_board, last_call) = process_call(bingo_calls, boards);

    winning_board.unused_spaces() * last_call
}

fn part_two_solution(bingo_calls: &Vec<i32>, boards: &Vec<BingoBoard>) -> i32 {
    // same as part one, but when a board wins, we take it out of the running
    // the last board to win is the winner
    // if i just look for the last winning board when I finish I'll find all the boards that have
    // already won, which is not useful

    0
}

fn read_and_process_input() -> (Vec<i32>, Vec<BingoBoard>) {
    process_input(read())
}

fn read() -> &'static str {
    include_str!("../data/puzzle_data")
}

fn process_input(data: &'static str) -> (Vec<i32>, Vec<BingoBoard>) {
    let mut input_data: Vec<&str> = data.split("\n\n").collect();

    let call_string = input_data.remove(0);
    let calls: Vec<i32> = call_string
        .trim()
        .split(",")
        .into_iter()
        .map(|call| call.parse::<i32>().unwrap())
        .collect();

    let boards: Vec<BingoBoard> = input_data
        .into_iter()
        .map(|board_layout| BingoBoard::new(board_layout))
        .collect();

    (calls, boards)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> (Vec<i32>, Vec<BingoBoard>) {
        let data = include_str!("../data/test_data");

        process_input(data)
    }

    #[test]
    fn test_part_one_example() {
        let data = test_data();
        assert_eq!(part_one_solution(&data.0, &data.1), 4512);
    }

    #[test]
    fn test_part_one_solution() {
        let data = read_and_process_input();

        assert_eq!(part_one_solution(&data.0, &data.1), 44736);
    }

    #[test]
    fn test_part_two_example() {
        let data = test_data();

        assert_eq!(part_two_solution(&data.0, &data.1), 1924);
    }
}
