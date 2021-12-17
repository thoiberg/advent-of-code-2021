use std::collections::HashMap;
use std::ops::Range;

fn main() {
    let lines = process_input(read_input());

    let part_one_answer = part_one_solution(&lines);
    println!("Part One Solution is: {}", part_one_answer);
}

// process data
// discard any that aren't horizontal or vertical
// loop through the rest and create hash of points (combine the x+y into a single variable) (or check out nd_array????)
//   then increment the variable
// loop through all final fields and count the fields that have 2 or more lines

fn part_one_solution(lines: &Vec<Line>) -> u32 {
    let valid_lines: Vec<&Line> = lines
        .into_iter()
        .filter(|line| line.is_horizontal() || line.is_vertical())
        .collect();

    let mut position_hash: HashMap<String, u32> = HashMap::new();

    for line in valid_lines {
        for point in line.each_point() {
            let id = format!("{}_{}", point.x, point.y);

            match position_hash.get(&id) {
                Some(val) => {
                    position_hash.insert(id, val + 1);
                }
                None => {
                    position_hash.insert(id, 1);
                }
            }
        }
    }

    position_hash
        .into_iter()
        .fold(0, |acc, (_, value)| if value > 1 { acc + 1 } else { acc })
}

struct Point {
    x: u32,
    y: u32,
}

impl Point {
    pub fn new(coordinates: &str) -> Point {
        let coords: Vec<&str> = coordinates.split(",").collect();

        Point {
            x: coords[0].parse::<u32>().unwrap(),
            y: coords[1].parse::<u32>().unwrap(),
        }
    }
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_vertical(&self) -> bool {
        self.start.y == self.end.y
    }

    fn each_point(&self) -> Box<dyn Iterator<Item = Point> + '_> {
        if self.is_horizontal() {
            let range: Range<u32>;

            if self.start.y < self.end.y {
                range = self.start.y..(self.end.y + 1);
            } else {
                range = self.end.y..(self.start.y + 1);
            }

            Box::new(range.into_iter().map(|value| Point {
                x: self.start.x,
                y: value,
            }))
        } else {
            let range: Range<u32>;

            if self.start.x < self.end.x {
                range = self.start.x..(self.end.x + 1);
            } else {
                range = self.end.x..(self.start.x + 1);
            }

            Box::new(range.into_iter().map(|value| Point {
                x: value,
                y: self.start.y,
            }))
        }
    }
}

fn read_input() -> &'static str {
    include_str!("../data/puzzle_data.txt")
}

fn process_input(input: &str) -> Vec<Line> {
    input
        .split("\n")
        .map(|line| {
            let points: Vec<&str> = line.split(" -> ").collect();
            Line {
                start: Point::new(points[0]),
                end: Point::new(points[1]),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<Line> {
        let input = include_str!("../data/test_data.txt");
        process_input(input)
    }

    #[test]
    fn test_part_one_example() {
        assert_eq!(part_one_solution(&test_data()), 5);
    }

    #[test]
    fn test_part_one_solution() {
        assert_eq!(part_one_solution(&process_input(read_input())), 5147);
    }
}
