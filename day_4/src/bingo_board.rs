#[derive(Clone, Debug)]
pub struct BingoBoard {
    bingo_spaces: Vec<Vec<BingoSpace>>,
}

impl BingoBoard {
    pub fn new(board_layout: &str) -> BingoBoard {
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

    pub fn mark(&mut self, number: i32) -> bool {
        for row in &mut self.bingo_spaces {
            for mut space in row {
                if space.value == number {
                    space.marked = true;
                }
            }
        }

        self.check()
    }

    pub fn check(&self) -> bool {
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

    pub fn unused_spaces(&self) -> i32 {
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
pub struct BingoSpace {
    value: i32,
    marked: bool,
}
