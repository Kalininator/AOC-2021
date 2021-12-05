use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point(i32, i32);

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<i32> = s.split(',').map(|c| c.parse().unwrap()).collect();
        Ok(Point(values[0], values[1]))
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }
    fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }

    fn all_points(&self, include_diagonals: bool) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];
        if self.is_vertical() {
            if self.start.1 < self.end.1 {
                for i in self.start.1..=self.end.1 {
                    points.push(Point(self.start.0, i));
                }
            } else {
                for i in self.end.1..=self.start.1 {
                    points.push(Point(self.start.0, i));
                }
            }
        }
        if self.is_horizontal() {
            if self.start.0 < self.end.0 {
                for i in self.start.0..=self.end.0 {
                    points.push(Point(i, self.start.1));
                }
            } else {
                for i in self.end.0..=self.start.0 {
                    points.push(Point(i, self.start.1));
                }
            }
        }
        points
    }
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sides: Vec<&str> = s.split(" -> ").collect();
        Ok(Line {
            start: sides[0].parse().unwrap(),
            end: sides[1].parse().unwrap(),
        })
    }
}

fn count_overlaps(points: &[Point]) -> u32 {
    let mut points_hash = HashMap::new();
    for point in points {
        let entry = points_hash.entry(point).or_insert(0);
        *entry += 1;
    }

    let mut overlaps: u32 = 0;
    for (_key, value) in points_hash {
        if value > 1 {
            overlaps += 1;
        }
    }
    overlaps
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_lines = utils::read_file(&args[1]);

    let lines: Vec<Line> = file_lines
        .iter()
        .map(|l| l.parse().unwrap())
        .filter(|l: &Line| l.is_vertical() | l.is_horizontal())
        .collect();
    let all_line_points: Vec<Point> = lines
        .iter()
        .flat_map(|line| line.all_points(false))
        .collect();
    let part_1_overlaps = count_overlaps(&all_line_points);
    println!("Overlaps: {}", part_1_overlaps);
}

#[test]
fn parse_line() {
    let input = "0,9 -> 5,9";
    let line: Line = input.parse().unwrap();
    assert_eq!(
        line,
        Line {
            start: Point(0, 9),
            end: Point(5, 9)
        }
    );
}

// #[test]
// fn get_all_points() {
//     let input = "0,9 -> 5,9";
//     let line: Line = input.parse().unwrap();
//     let points = line.all_points();
// }

// #[test]
// fn line_does_collide() {
//     let l1: Line = "0,9 -> 5,9".parse().unwrap();
//     let l2: Line = "0,9 -> 2,9".parse().unwrap();
//     assert_eq!(lines_intersect(&l1, &l2).is_none(), true);
// }
