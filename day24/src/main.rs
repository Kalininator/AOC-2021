use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;
type Cache = HashMap<(i64, usize), Option<i64>>;

#[derive(Clone, Copy)]
enum Source {
    Reg(usize),
    Val(i64),
}

impl Source {
    fn val(&self, regs: &[i64; 4]) -> i64 {
        match *self {
            Self::Reg(i) => regs[i],
            Self::Val(v) => v,
        }
    }
}

impl FromStr for Source {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "w" => Self::Reg(0),
            "x" => Self::Reg(1),
            "y" => Self::Reg(2),
            "z" => Self::Reg(3),
            _ => Self::Val(s.parse()?),
        })
    }
}

#[derive(Clone, Copy)]
enum Instruction {
    Input(usize),
    Add(usize, Source),
    Multiply(usize, Source),
    Divide(usize, Source),
    Modulo(usize, Source),
    Equal(usize, Source),
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(l: &str) -> Result<Self, Self::Err> {
        let src = match l[4..5].parse::<Source>()? {
            Source::Reg(src) => src,
            _ => unreachable!(),
        };
        Ok(match &l[..3] {
            "inp" => Instruction::Input(src),
            "add" => Instruction::Add(src, l[6..].parse()?),
            "mul" => Instruction::Multiply(src, l[6..].parse()?),
            "div" => Instruction::Divide(src, l[6..].parse()?),
            "mod" => Instruction::Modulo(src, l[6..].parse()?),
            "eql" => Instruction::Equal(src, l[6..].parse()?),
            _ => unreachable!(),
        })
    }
}

fn find_model_number(
    cache: &mut Cache,
    blocks: &[Vec<Instruction>],
    block: usize,
    z: i64,
    range: &[i64; 9],
) -> Option<i64> {
    if let Some(&answer) = cache.get(&(z, block)) {
        return answer;
    }

    for &digit in range {
        let mut regs = [digit, 0, 0, z];
        for &inst in &blocks[block] {
            match inst {
                Instruction::Add(a, b) => regs[a] += b.val(&regs),
                Instruction::Multiply(a, b) => regs[a] *= b.val(&regs),
                Instruction::Divide(a, b) => regs[a] /= b.val(&regs),
                Instruction::Modulo(a, b) => regs[a] %= b.val(&regs),
                Instruction::Equal(a, b) => regs[a] = (regs[a] == b.val(&regs)) as i64,
                Instruction::Input(_) => unreachable!(),
            }
        }
        let z = regs[3];
        if block + 1 == blocks.len() {
            if z == 0 {
                cache.insert((z, block), Some(digit));
                return Some(digit);
            }
            continue;
        }
        if let Some(best) = find_model_number(cache, blocks, block + 1, z, range) {
            cache.insert((z, block), Some(best * 10 + digit));
            return Some(best * 10 + digit);
        }
    }

    cache.insert((z, block), None);
    None
}

fn solve(blocks: &[Vec<Instruction>], biggest: bool) -> String {
    let mut digits = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    if biggest {
        digits.reverse();
    }
    let answer = find_model_number(&mut Cache::new(), blocks, 0, 0, &digits).unwrap();
    answer.to_string().chars().rev().collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let lines = utils::read_file(&args[1]);
    let instructions = lines
        .iter()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<Instruction>>();
    let blocks = instructions
        .chunks(18)
        .map(|c| c.iter().skip(1).copied().collect())
        .collect::<Vec<_>>();
    println!("Part one: {}", solve(&blocks, true));
    println!("Part two: {}", solve(&blocks, false));
}
