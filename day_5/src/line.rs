use std::ops::RangeInclusive;

pub struct Point {
    pub x: u32,
    pub y: u32,
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

pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn is_horizontal(&self) -> bool {
        self.start.x == self.end.x
    }

    pub fn is_vertical(&self) -> bool {
        self.start.y == self.end.y
    }

    pub fn each_point(&self) -> Box<dyn Iterator<Item = Point> + '_> {
        if self.is_horizontal() {
            let range: RangeInclusive<u32>;

            if self.start.y < self.end.y {
                range = RangeInclusive::new(self.start.y, self.end.y);
            } else {
                range = RangeInclusive::new(self.end.y, self.start.y);
            }

            Box::new(range.into_iter().map(|value| Point {
                x: self.start.x,
                y: value,
            }))
        } else {
            let range: RangeInclusive<u32>;

            if self.start.x < self.end.x {
                range = RangeInclusive::new(self.start.x, self.end.x);
            } else {
                range = RangeInclusive::new(self.end.x, self.start.x);
            }

            Box::new(range.into_iter().map(|value| Point {
                x: value,
                y: self.start.y,
            }))
        }
    }
}
