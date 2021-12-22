use std::cmp::{max, min};
use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Hash, PartialEq, Eq)]
struct Point {
    x: i128,
    y: i128,
    z: i128,
}

impl Point {
    fn new(x: i128, y: i128, z: i128) -> Self {
        Point { x, y, z }
    }
}

#[derive(Debug)]
struct Step {
    turn_on: bool,
    x1: i128,
    x2: i128,
    y1: i128,
    y2: i128,
    z1: i128,
    z2: i128,
}

impl FromStr for Step {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (turn_on, x1, x2, y1, y2, z1, z2) = sscanf::scanf!(
            s,
            "{} x={}..{},y={}..{},z={}..{}",
            String,
            i128,
            i128,
            i128,
            i128,
            i128,
            i128,
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

impl Step {
    fn cuboid(&self) -> Cuboid {
        Cuboid(
            Range(self.x1, self.x2),
            Range(self.y1, self.y2),
            Range(self.z1, self.z2),
        )
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

#[derive(Clone, Copy)]
struct Range(i128, i128);
impl Range {
    fn len(&self) -> i128 {
        self.1 - self.0 + 1
    }
}
#[derive(Clone, Copy)]
struct Cuboid(Range, Range, Range);
impl Cuboid {
    fn new(x_min: i128, x_max: i128, y_min: i128, y_max: i128, z_min: i128, z_max: i128) -> Self {
        Cuboid(
            Range(x_min, x_max),
            Range(y_min, y_max),
            Range(z_min, z_max),
        )
    }

    fn volume(&self) -> i128 {
        self.0.len() * self.1.len() * self.2.len()
    }

    fn split_intersection(&mut self, other: &Cuboid) -> Vec<Cuboid> {
        let mut result_vec = Vec::new();
        if (self.0 .0 <= other.0 .1 && self.0 .1 >= other.0 .0)
            && (self.1 .0 <= other.1 .1 && self.1 .1 >= other.1 .0)
            && (self.2 .0 <= other.2 .1 && self.2 .1 >= other.2 .0)
        {
            // on x
            if self.0 .0 < other.0 .0 {
                result_vec.push(Cuboid::new(
                    self.0 .0,
                    other.0 .0 - 1,
                    self.1 .0,
                    self.1 .1,
                    self.2 .0,
                    self.2 .1,
                ));
                self.0 .0 = other.0 .0;
            }
            if self.0 .1 > other.0 .1 {
                result_vec.push(Cuboid::new(
                    other.0 .1 + 1,
                    self.0 .1,
                    self.1 .0,
                    self.1 .1,
                    self.2 .0,
                    self.2 .1,
                ));
                self.0 .1 = other.0 .1;
            }
            // on y
            if self.1 .0 < other.1 .0 {
                result_vec.push(Cuboid::new(
                    self.0 .0,
                    self.0 .1,
                    self.1 .0,
                    other.1 .0 - 1,
                    self.2 .0,
                    self.2 .1,
                ));
                self.1 .0 = other.1 .0;
            }
            if self.1 .1 > other.1 .1 {
                result_vec.push(Cuboid::new(
                    self.0 .0,
                    self.0 .1,
                    other.1 .1 + 1,
                    self.1 .1,
                    self.2 .0,
                    self.2 .1,
                ));
                self.1 .1 = other.1 .1;
            }
            // on z
            if self.2 .0 < other.2 .0 {
                result_vec.push(Cuboid::new(
                    self.0 .0,
                    self.0 .1,
                    self.1 .0,
                    self.1 .1,
                    self.2 .0,
                    other.2 .0 - 1,
                ));
                self.2 .0 = other.2 .0;
            }
            if self.2 .1 > other.2 .1 {
                result_vec.push(Cuboid::new(
                    self.0 .0,
                    self.0 .1,
                    self.1 .0,
                    self.1 .1,
                    other.2 .1 + 1,
                    self.2 .1,
                ));
                self.2 .1 = other.2 .1;
            }
        } else {
            result_vec.push(*self)
        }
        result_vec
    }
}

///              X+
///          Y+ /
///          | /
///          |/
///   Z+ ----------- Z-
///         /|
///        / |
///       /  Y-
///     X-

fn part_two(steps: &[Step]) -> i128 {
    // let mut cuboids: Vec<Cuboid> = vec![];
    let cuboids = steps.iter().fold(Vec::<Cuboid>::new(), |mut acc, step| {
        let mut cuboids: Vec<Cuboid> = Vec::with_capacity(acc.len() + 24);
        let parsed_cuboid = step.cuboid();
        for oc in acc.iter_mut() {
            cuboids.append(&mut oc.split_intersection(&parsed_cuboid));
        }
        if step.turn_on {
            cuboids.push(parsed_cuboid);
        }
        cuboids
    });
    cuboids.iter().map(|c| c.volume()).sum()
}

fn part1(v: &[Cuboid]) -> i128 {
    v.iter()
        .filter(|c| {
            c.0 .0 >= -50
                && c.0 .1 <= 50
                && c.1 .0 >= -50
                && c.1 .1 <= 50
                && c.2 .0 >= -50
                && c.2 .1 <= 50
        })
        .map(|c| c.volume())
        .sum()
}

fn parse(steps: &[Step]) -> Vec<Cuboid> {
    steps.iter().fold(Vec::<Cuboid>::new(), |mut acc, step| {
        let mut cuboids: Vec<Cuboid> = Vec::with_capacity(acc.len() + 24);
        let parsed_cuboid = step.cuboid();
        for oc in acc.iter_mut() {
            cuboids.append(&mut oc.split_intersection(&parsed_cuboid));
        }
        if step.turn_on {
            cuboids.push(parsed_cuboid);
        }
        cuboids
    })
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let lines = utils::read_file(&args[1]);
    let steps: Vec<Step> = lines.iter().map(|l| l.parse().unwrap()).collect();
    let cuboids = parse(&steps);
    println!("Part one: {}", part_one(&steps));
    println!("Part one: {}", part1(&cuboids));
    println!("Part two: {}", part_two(&steps));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_area() {
        assert_eq!(Range(3, 5).len(), 3);
    }

    #[test]
    fn cuboid_volume() {
        assert_eq!(Cuboid(Range(0, 2), Range(0, 2), Range(0, 2)).volume(), 27);
    }
}
