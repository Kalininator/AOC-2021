fn main() {
    let args: Vec<String> = std::env::args().collect();
    let lines = utils::read_file(&args[1]);
    println!("Part one: {}", part_one(&lines));
}

fn first_illegal(line: &str) -> u32 {
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
                    return 3;
                }
            }
            ']' => {
                let last = chars.pop().unwrap();
                if last != '[' {
                    return 57;
                }
            }
            '}' => {
                let last = chars.pop().unwrap();
                if last != '{' {
                    return 1197;
                }
            }
            '>' => {
                let last = chars.pop().unwrap();
                if last != '<' {
                    return 25137;
                }
            }
            _ => panic!("fo"),
        }
    }
    0
}

fn part_one(lines: &[String]) -> u32 {
    lines.iter().map(|l| first_illegal(l)).sum()
}

#[test]
fn first_illegal_square() {
    let input = "[[<[([]))<([[{}[[()]]]";
    assert_eq!(first_illegal(input), 3);
}
