use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let lines = utils::read_file(&args[1]);
    let pairs: Vec<SnailFish> = lines.iter().map(|l| l.parse().unwrap()).collect();
    println!("Part 1 Magnitude: {}", part_one(&pairs));
    println!("Part 2 Magnitude: {}", part_two(&pairs));
}

fn part_one(fish: &[SnailFish]) -> usize {
    let added_pair = fish
        .iter()
        .skip(1)
        .fold(fish[0].clone(), |acc, next| add_and_reduce(&acc, next));
    added_pair.magnitude()
}

fn part_two(fish: &[SnailFish]) -> usize {
    let mut max = 0usize;
    for x in 0..fish.len() {
        for y in 0..fish.len() {
            if x == y {
                continue;
            }
            let xy = add_and_reduce(&fish[x], &fish[y]).magnitude();
            if xy > max {
                max = xy
            }
            let yx = add_and_reduce(&fish[y], &fish[x]).magnitude();
            if yx > max {
                max = yx
            }
        }
    }
    max
}

#[derive(Debug, Clone, PartialEq)]
enum SnailFish {
    Value(u128),
    Pair {
        left: Box<SnailFish>,
        right: Box<SnailFish>,
    },
}

impl FromStr for SnailFish {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if &s[..1] != "[" {
            return Ok(SnailFish::Value(s.parse()?));
        } else {
            let mut encountered_brackets = 0u128;
            let chars: Vec<char> = s.chars().collect();
            for (i, c) in chars.iter().enumerate().take(s.len() - 1).skip(1) {
                match c {
                    '[' => encountered_brackets += 1,
                    ']' => encountered_brackets -= 1,
                    _ => {}
                };
                if encountered_brackets == 0 {
                    let comma_index = i + 1;
                    return Ok(SnailFish::Pair {
                        left: Box::new(s[1..comma_index].parse()?),
                        right: Box::new(s[comma_index + 1..(s.len() - 1)].parse()?),
                    });
                }
            }
        };
        unreachable!()
    }
}

impl fmt::Display for SnailFish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SnailFish::Value(val) => write!(f, "{}", val),
            SnailFish::Pair { left, right } => write!(f, "[{},{}]", left, right),
        }
    }
}

impl SnailFish {
    fn magnitude(&self) -> usize {
        match self {
            SnailFish::Value(val) => *val as usize,
            SnailFish::Pair { left, right } => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }

    fn absorb(&mut self, from_left: bool, val: u128) {
        match self {
            SnailFish::Value(prev) => *prev += val,
            SnailFish::Pair { left, right } => match from_left {
                true => left.absorb(from_left, val),
                false => right.absorb(from_left, val),
            },
        }
    }

    fn reduce(&mut self, depth: u128) -> Option<(u128, u128)> {
        match self {
            SnailFish::Value(_) => None,
            SnailFish::Pair { left, right } => {
                if depth == 4 {
                    let a = match **left {
                        SnailFish::Value(val) => val,
                        _ => unreachable!(),
                    };
                    let b = match **right {
                        SnailFish::Value(val) => val,
                        _ => unreachable!(),
                    };
                    *self = SnailFish::Value(0);
                    Some((a, b))
                } else {
                    if let Some((a, b)) = left.reduce(depth + 1) {
                        right.absorb(true, b);
                        return Some((a, 0));
                    }
                    if let Some((a, b)) = right.reduce(depth + 1) {
                        left.absorb(false, a);
                        return Some((0, b));
                    }
                    None
                }
            }
        }
    }

    fn split(&mut self) -> Option<()> {
        match self {
            SnailFish::Value(val) => {
                if *val >= 10 {
                    *self = SnailFish::Pair {
                        left: Box::new(SnailFish::Value((*val as f32 / 2.0).floor() as u128)),
                        right: Box::new(SnailFish::Value((*val as f32 / 2.0).ceil() as u128)),
                    };
                    Some(())
                } else {
                    None
                }
            }
            SnailFish::Pair { left, right } => {
                if left.split().is_some() {
                    return Some(());
                }
                if right.split().is_some() {
                    return Some(());
                }
                None
            }
        }
    }
}

fn add_and_reduce(x: &SnailFish, y: &SnailFish) -> SnailFish {
    let mut res = SnailFish::Pair {
        left: Box::new(x.clone()),
        right: Box::new(y.clone()),
    };
    while res.reduce(0).is_some() || res.split().is_some() {}
    res
}
