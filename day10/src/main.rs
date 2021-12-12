fn main() {
    let args: Vec<String> = std::env::args().collect();
    let lines = utils::read_file(&args[1]);
    println!("Part one: {}", part_one(&lines));
    println!("Part two: {}", part_two(&lines));
}

#[derive(Debug, PartialEq)]
enum Score {
    Illegal(u128),
    Incomplete(u128),
}

fn first_illegal(line: &str) -> Score {
    let mut chars: Vec<char> = vec![];

    for char in line.chars() {
        match char {
            '(' => chars.push('('),
            '[' => chars.push('['),
            '{' => chars.push('{'),
            '<' => chars.push('<'),
            ')' => {
                let last = chars.pop().unwrap();
                if last != '(' {
                    return Score::Illegal(3);
                }
            }
            ']' => {
                let last = chars.pop().unwrap();
                if last != '[' {
                    return Score::Illegal(57);
                }
            }
            '}' => {
                let last = chars.pop().unwrap();
                if last != '{' {
                    return Score::Illegal(1197);
                }
            }
            '>' => {
                let last = chars.pop().unwrap();
                if last != '<' {
                    return Score::Illegal(25137);
                }
            }
            _ => panic!("fo"),
        }
    }
    let mut acc: u128 = 0;
    for c in chars.iter().rev() {
        acc *= 5;
        acc += match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("foo"),
        }
    }
    Score::Incomplete(acc)
}

fn part_one(lines: &[String]) -> u128 {
    let mut acc: u128 = 0;
    for score in lines.iter().map(|l| first_illegal(l)) {
        if let Score::Illegal(val) = score {
            acc += val
        }
    }
    acc
}

fn part_two(lines: &[String]) -> u128 {
    let mut scores: Vec<u128> = lines
        .iter()
        .map(|l| first_illegal(l))
        .filter_map(|score| {
            if let Score::Incomplete(val) = score {
                return Some(val);
            }
            None
        })
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[test]
fn first_illegal_square() {
    let input = "[[<[([]))<([[{}[[()]]]";
    assert_eq!(first_illegal(input), Score::Illegal(3));
}
