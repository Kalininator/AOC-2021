use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let lines = utils::read_file(&args[1]);
    for line in lines {
        let sf: SnailFish = line.parse().unwrap();
        println!("{:?}", sf);
        println!("{}", sf);
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum SnailFishValue {
    Number(u32),
    SnailFish(Box<SnailFish>),
}

impl FromStr for SnailFishValue {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if &s[..1] == "[" {
            // Nested snailfish
            return Ok(SnailFishValue::SnailFish(Box::new(s.parse()?)));
        }
        Ok(SnailFishValue::Number(s.parse()?))
    }
}

impl fmt::Display for SnailFishValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SnailFishValue::Number(val) => write!(f, "{}", val),
            SnailFishValue::SnailFish(fish) => write!(f, "{}", fish),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SnailFish {
    left: SnailFishValue,
    right: SnailFishValue,
}

impl FromStr for SnailFish {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // First char is [
        // Last char is ]
        // Need to find the ,
        let mut encountered_brackets = 0u32;
        let chars: Vec<char> = s.chars().collect();
        for i in 1..(s.len() - 1) {
            match chars[i] {
                '[' => encountered_brackets += 1,
                ']' => encountered_brackets -= 1,
                _ => {}
            };
            if encountered_brackets == 0 {
                // Next character should be a comma
                let comma_index = i + 1;
                return Ok(SnailFish {
                    left: s[1..comma_index].parse()?,
                    right: s[comma_index + 1..(s.len() - 1)].parse()?,
                });
            };
        }
        panic!("yikes");
    }
}

impl fmt::Display for SnailFish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{}]", self.left, self.right)
    }
}

fn add_snailfish(left: SnailFish, right: SnailFish) -> SnailFish {
    SnailFish {
        left: SnailFishValue::SnailFish(Box::new(left)),
        right: SnailFishValue::SnailFish(Box::new(right)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_addition() {
        let left: SnailFish = "[1,2]".parse().unwrap();
        let right: SnailFish = "[[3,4],5]".parse().unwrap();
        assert_eq!(
            format!("{}", add_snailfish(left, right)),
            "[[1,2],[[3,4],5]]"
        );
    }
}
