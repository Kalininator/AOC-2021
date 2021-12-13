use regex::Regex;
use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}
impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<u32> = s.split(',').map(|n| n.parse().unwrap()).collect();
        Ok(Point {
            x: split[0],
            y: split[1],
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Axis {
    X,
    Y,
}

#[derive(Debug, PartialEq, Eq)]
struct Fold {
    axis: Axis,
    line: u32,
}

impl FromStr for Fold {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fold_regex = Regex::new(r"fold along ([xy])=(\d*)").unwrap();
        let captures = fold_regex.captures(s).unwrap();
        let axis: Axis = match captures.get(1).unwrap().as_str() {
            "x" => Axis::X,
            "y" => Axis::Y,
            _ => panic!("invalid axis"),
        };
        let line: u32 = captures.get(2).unwrap().as_str().parse()?;
        Ok(Fold { axis, line })
    }
}

fn fold_points(points: &mut Vec<Point>, fold: &Fold) {
    for p in points {
        match fold.axis {
            Axis::Y => {
                if p.y > fold.line {
                    p.y = fold.line - (p.y - fold.line);
                }
            }
            Axis::X => {
                if p.x > fold.line {
                    p.x = fold.line - (p.x - fold.line);
                }
            }
        }
    }
}

fn render_points(points: &[Point]) {
    let width = points.to_owned().iter().map(|p| p.x).max().unwrap();
    let height = points.to_owned().iter().map(|p| p.y).max().unwrap();
    for y in 0..=height {
        for x in 0..=width {
            if points.contains(&Point { x, y }) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn dedupe_points(points: &mut Vec<Point>) {
    let set: HashSet<_> = points.drain(..).collect();
    points.extend(set.into_iter());
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_lines = utils::read_file(&args[1]);

    let mut points: Vec<Point> = vec![];
    let mut folds: Vec<Fold> = vec![];

    let mut finding_points = true;
    for line in file_lines {
        if line.is_empty() {
            finding_points = false;
            continue;
        }
        if finding_points {
            points.push(line.parse().unwrap());
        } else {
            folds.push(line.parse().unwrap());
        }
    }
    for (i, fold) in folds.iter().enumerate() {
        fold_points(&mut points, fold);
        dedupe_points(&mut points);
        if i == 0 {
            println!("Points after first fold: {}", points.len());
        }
    }
    render_points(&points);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_fold_y() {
        assert_eq!(
            "fold along y=7".parse::<Fold>().unwrap(),
            Fold {
                axis: Axis::Y,
                line: 7
            }
        );
    }

    #[test]
    fn parse_fold_x() {
        assert_eq!(
            "fold along x=27".parse::<Fold>().unwrap(),
            Fold {
                axis: Axis::X,
                line: 27
            }
        );
    }
}
