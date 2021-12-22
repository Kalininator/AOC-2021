use std::cmp::{max, min};
use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Point { x, y, z }
    }
}

#[derive(Debug)]
struct Step {
    turn_on: bool,
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
    z1: i32,
    z2: i32,
}

impl FromStr for Step {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (turn_on, x1, x2, y1, y2, z1, z2) = sscanf::scanf!(
            s,
            "{} x={}..{},y={}..{},z={}..{}",
            String,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
        )
        .unwrap();

        Ok(Step {
            turn_on: turn_on == "on",
            x1,
            x2,
            y1,
            y2,
            z1,
            z2,
        })
    }
}

fn part_one(steps: &[Step]) -> usize {
    let mut set: HashSet<Point> = HashSet::new();
    for s in steps {
        for x in max(s.x1, -50)..=min(s.x2, 50) {
            for y in max(s.y1, -50)..=min(s.y2, 50) {
                for z in max(s.z1, -50)..=min(s.z2, 50) {
                    match s.turn_on {
                        true => set.insert(Point::new(x, y, z)),
                        false => set.remove(&Point::new(x, y, z)),
                    };
                }
            }
        }
    }
    set.len()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let lines = utils::read_file(&args[1]);
    let steps: Vec<Step> = lines.iter().map(|l| l.parse().unwrap()).collect();
    println!("Part one: {}", part_one(&steps));
}
