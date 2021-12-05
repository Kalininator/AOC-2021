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

impl Line {
    fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }
    fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }

    fn is_diagonal(&self) -> bool {
        (self.start.0 - self.end.0).abs() == (self.start.1 - self.end.1).abs()
    }

    fn all_points(&self, include_diagonals: bool) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];
        if self.is_vertical() {
            let range = if self.start.1 < self.end.1 {
                self.start.1..=self.end.1
            } else {
                self.end.1..=self.start.1
            };
            for i in range {
                points.push(Point(self.start.0, i));
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
        if include_diagonals && self.is_diagonal() {
            let x_rev = self.start.0 < self.end.0;
            let x_range: Vec<i32> = if x_rev {
                (self.start.0..=self.end.0).collect()
            } else {
                (self.end.0..=self.start.0).rev().collect()
            };
            let y_rev = self.start.1 < self.end.1;
            let y_range: Vec<i32> = if y_rev {
                (self.start.1..=self.end.1).collect()
            } else {
                (self.end.1..=self.start.1).rev().collect()
            };
            for (x, y) in (x_range).iter().zip(y_range) {
                points.push(Point(*x, y));
            }
        }
        points
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

    let lines: Vec<Line> = file_lines.iter().map(|l| l.parse().unwrap()).collect();
    let all_line_points: Vec<Point> = lines
        .iter()
        .flat_map(|line| line.all_points(false))
        .collect();
    let part_1_overlaps = count_overlaps(&all_line_points);
    println!("Overlaps Part 1: {}", part_1_overlaps);
    let all_line_points_with_diagonals: Vec<Point> = lines
        .iter()
        .flat_map(|line| line.all_points(true))
        .collect();
    let part_2_overlaps = count_overlaps(&all_line_points_with_diagonals);
    println!("Overlaps Part 2: {}", part_2_overlaps);
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

#[test]
fn is_diagonal_test() {
    let input = "8,0 -> 0,8";
    let line: Line = input.parse().unwrap();
    assert!(line.is_diagonal());
}

#[test]
fn get_all_horizontal_points() {
    let input = "0,9 -> 2,9";
    let line: Line = input.parse().unwrap();
    let points = line.all_points(false);
    assert_eq!(points, vec![Point(0, 9), Point(1, 9), Point(2, 9)]);
}

#[test]
fn get_all_diagonal_points() {
    let input = "3,1 -> 1,3";
    let line: Line = input.parse().unwrap();
    let points = line.all_points(true);
    assert_eq!(points, vec![Point(3, 1), Point(2, 2), Point(1, 3)]);
}

// #[test]
// fn line_does_collide() {
//     let l1: Line = "0,9 -> 5,9".parse().unwrap();
//     let l2: Line = "0,9 -> 2,9".parse().unwrap();
//     assert_eq!(lines_intersect(&l1, &l2).is_none(), true);
// }
