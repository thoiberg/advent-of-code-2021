use std::ops::RangeInclusive;

#[derive(Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(coordinates: &str) -> Point {
        let coords: Vec<&str> = coordinates.split(",").collect();

        Point {
            x: coords[0].parse::<i32>().unwrap(),
            y: coords[1].parse::<i32>().unwrap(),
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Clone)]
pub struct Line {
    pub start: Point,
    pub end: Point,
    pub at_start: bool,
}

impl Line {
    pub fn is_horizontal(&self) -> bool {
        self.start.x == self.end.x
    }

    pub fn is_vertical(&self) -> bool {
        self.start.y == self.end.y
    }

    // no longer used but I'm keeping it since it could be a good reference for creating a
    // simple iterator function.
    pub fn each_point(&self) -> Box<dyn Iterator<Item = Point> + '_> {
        if self.is_horizontal() {
            let range: RangeInclusive<i32>;

            if self.start.y < self.end.y {
                range = RangeInclusive::new(self.start.y, self.end.y);
            } else {
                range = RangeInclusive::new(self.end.y, self.start.y);
            }

            Box::new(range.into_iter().map(|value| Point {
                x: self.start.x,
                y: value,
            }))
        } else if self.is_vertical() {
            let range: RangeInclusive<i32>;

            if self.start.x < self.end.x {
                range = RangeInclusive::new(self.start.x, self.end.x);
            } else {
                range = RangeInclusive::new(self.end.x, self.start.x);
            }

            Box::new(range.into_iter().map(|value| Point {
                x: value,
                y: self.start.y,
            }))
        } else {
            let range: RangeInclusive<i32>;

            if self.start.x < self.end.x {
                range = RangeInclusive::new(self.start.x, self.end.x);
            } else {
                range = RangeInclusive::new(self.end.x, self.start.x);
            }

            Box::new(range.into_iter().map(|value| {
                let new_x = value + offset(self.start.x, self.end.x);
                let new_y = value + offset(self.start.y, self.end.y);

                Point { x: new_x, y: new_y }
            }))
        }
    }
}

impl Iterator for Line {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        if self.start == self.end {
            None
        } else if self.at_start {
            // this is a real gross hack, but I can't figure out how to get the iterator
            // to iterate over the inclusive range. It either lops off the start or the end
            self.at_start = false;
            Some(self.start.clone())
        } else {
            let new_x = self.start.x + offset(self.start.x, self.end.x);
            let new_y = self.start.y + offset(self.start.y, self.end.y);

            self.start = Point { x: new_x, y: new_y };
            self.at_start = false;

            Some(Point { x: new_x, y: new_y })
        }
    }
}

fn offset(a: i32, b: i32) -> i32 {
    if a == b {
        0
    } else if a < b {
        1
    } else {
        -1
    }
}
