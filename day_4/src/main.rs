fn main() {
    let data = read_and_process_input();

    let part_one_answer = part_one_solution(data.0, &data.1);

    println!("Part One Solution is: {}", part_one_answer);

    let part_two_answer = part_two_solution(data.0, &data.1);

    println!("Part Two Solution is: {}", part_two_answer);
}

fn part_one_solution(bingo_calls: &'static str, boards: &Vec<BingoBoard>) -> i32 {
    let calls: Vec<&str> = bingo_calls.trim().split(",").collect();
    let mut answer = 0;
    let mut mut_boards = boards.to_owned();

    for call in calls {
        let call_int = call.parse::<i32>().unwrap();
        for board in &mut mut_boards {
            let winner = board.mark(call_int);

            if winner.to_owned() {
                answer = board.unused_spaces() * call_int
            }
        }

        if answer > 0 {
            return answer;
        }
    }

    answer
}

fn part_two_solution(bingo_calls: &'static str, boards: &Vec<BingoBoard>) -> i32 {
    // same as part one, but when a board wins, we take it out of the running
    // the last board to win is the winner
    // if i just look for the last winning board when I finish I'll find all the boards that have
    // already won, which is not useful

    0
}

fn read_and_process_input() -> (&'static str, Vec<BingoBoard>) {
    process_input(read())
}

fn read() -> &'static str {
    include_str!("../data/puzzle_data")
}

fn process_input(data: &'static str) -> (&'static str, Vec<BingoBoard>) {
    let mut input_data: Vec<&str> = data.split("\n\n").collect();

    let bingo_call = input_data.remove(0);

    let boards: Vec<BingoBoard> = input_data
        .into_iter()
        .map(|board_layout| BingoBoard::new(board_layout))
        .collect();

    (bingo_call, boards)
}

#[derive(Clone, Debug)]
struct BingoBoard {
    bingo_spaces: Vec<Vec<BingoSpace>>,
}

impl BingoBoard {
    fn new(board_layout: &str) -> BingoBoard {
        let rows: Vec<&str> = board_layout.split("\n").collect();

        let board_spaces: Vec<Vec<BingoSpace>> = rows
            .into_iter()
            .map(|row| {
                let spaces: Vec<&str> = row.trim().split(" ").filter(|x| x.len() > 0).collect();

                let bingo_spaces: Vec<BingoSpace> = spaces
                    .into_iter()
                    .map(|space| BingoSpace {
                        value: space.trim().parse::<i32>().unwrap(),
                        marked: false,
                    })
                    .collect();

                bingo_spaces
            })
            .collect();

        BingoBoard {
            bingo_spaces: board_spaces,
        }
    }

    fn mark(&mut self, number: i32) -> bool {
        for row in &mut self.bingo_spaces {
            for mut space in row {
                if space.value == number {
                    space.marked = true;
                }
            }
        }

        // TODO: Remove .to_vec() (I should be able to do this without having to take ownership??)
        let row_win = &self
            .bingo_spaces
            .to_vec()
            .into_iter()
            .any(|row| row.into_iter().all(|space| space.marked));

        // TODO: Remove .to_vec() (I should be able to do this without having to take ownership??)
        let row_length = self.bingo_spaces.first().unwrap().len();
        let column_win = (0..(row_length - 1)).into_iter().any(|column| {
            let val = &self
                .bingo_spaces
                .to_vec()
                .into_iter()
                .all(|row| row[column].marked);

            return val.to_owned();
        });

        row_win.to_owned() || column_win.to_owned()
    }

    fn unused_spaces(&self) -> i32 {
        let sum = &self
            .bingo_spaces
            .to_vec()
            .into_iter()
            .flatten()
            .fold(
                0,
                |acc, space| if space.marked { acc } else { acc + space.value },
            );

        *sum
    }
}

#[derive(Clone, Debug)]
struct BingoSpace {
    value: i32,
    marked: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> (&'static str, Vec<BingoBoard>) {
        let data = include_str!("../data/test_data");

        process_input(data)
    }

    #[test]
    fn test_part_one_example() {
        let data = test_data();
        assert_eq!(part_one_solution(data.0, &data.1), 4512);
    }

    #[test]
    fn test_part_one_solution() {
        let data = read_and_process_input();

        assert_eq!(part_one_solution(data.0, &data.1), 44736);
    }
}
