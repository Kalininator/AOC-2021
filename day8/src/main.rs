use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Pattern {
    text: String,
    value: Option<u8>,
}

impl Pattern {
    fn new(text: &str) -> Self {
        Pattern {
            text: sort_chars(text),
            value: try_match_value(text),
        }
    }
}

fn sort_chars(text: &str) -> String {
    let mut chars: Vec<char> = text.chars().collect();
    chars.sort_unstable();
    String::from_iter(chars)
}

fn try_match_value(text: &str) -> Option<u8> {
    match text.len() {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        _ => None,
    }
}

fn create_hashmap(patterns: &[Pattern]) -> HashMap<String, u8> {
    let mut map: HashMap<String, u8> = HashMap::new();
    for p in patterns {
        if let Some(val) = p.value {
            map.insert(p.text.clone(), val);
        }
    }
    map
}

fn parse_line(line: &str) -> (Vec<Pattern>, Vec<String>) {
    let sections: Vec<&str> = line.split('|').collect();
    let patterns: Vec<Pattern> = sections[0]
        .split_whitespace()
        .map(|s| Pattern::new(s))
        .collect();
    let output_values: Vec<String> = sections[1].split_whitespace().map(sort_chars).collect();
    (patterns, output_values)
}

fn part_one(lines: &[String]) {
    let mut acc: u32 = 0;
    for line in lines {
        let (patterns, digits) = parse_line(line);
        let map = create_hashmap(&patterns);
        for d in digits {
            if let Some(_value) = map.get(&d) {
                acc += 1;
            }
        }
    }
    println!("Part one: {}", acc);
}

fn find_value(map: &HashMap<String, u8>, value: u8) -> Option<&String> {
    map.iter()
        .find_map(|(key, &val)| if val == value { Some(key) } else { None })
}

fn decipher_line(line: &str) -> u32 {
    let (patterns, digits) = parse_line(line);
    let mut map = create_hashmap(&patterns);
    let patterns: Vec<Pattern> = patterns
        .iter()
        .map(|p| {
            if p.value.is_none()
                && p.text.len() == 5
                && p.text.contains(find_value(&map, 1).unwrap())
            {
                map.insert(p.text.clone(), 3);
                return Pattern {
                    text: p.text.clone(),
                    value: Some(3),
                };
            }
            Pattern {
                text: p.text.clone(),
                value: None,
            }
        })
        .collect();
    1
}

fn parse<'a>(input: &'a str) -> Vec<(Vec<&'a str>, Vec<&'a str>)> {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts: Vec<Vec<&str>> = line
                .split(" | ")
                .map(|strings| strings.split_whitespace().collect::<Vec<_>>())
                .collect();
            let b = parts.pop().unwrap();
            let a = parts.pop().unwrap();
            (a, b)
        })
        .collect()
}

fn part_two() {
    let input: &str = include_str!("../input.txt");
    let lines = parse(input);
    let mut sum = 0;

    for (mut patterns, output) in lines {
        let mut mappings = vec![String::new(); 10];

        // search for uniques first
        patterns.retain(|pat| match pat.len() {
            2 => {
                mappings[1] = pat.to_string();
                false
            }

            4 => {
                mappings[4] = pat.to_string();
                false
            }

            3 => {
                mappings[7] = pat.to_string();
                false
            }

            7 => {
                mappings[8] = pat.to_string();
                false
            }

            _ => true,
        });

        // we can deduce 9 easily: it contains both 4 and 7 and has length 6; we can also deduce 3, since it has 1
        patterns.retain(|pat| {
            if pat.len() == 6
                && mappings[4].chars().all(|seg| pat.contains(seg))
                && mappings[7].chars().all(|seg| pat.contains(seg))
            {
                mappings[9] = pat.to_string();
                false
            } else if pat.len() == 5 && mappings[1].chars().all(|seg| pat.contains(seg)) {
                mappings[3] = pat.to_string();
                false
            } else {
                true
            }
        });

        // 2: 9 doesn’t contain it
        patterns.retain(|pat| {
            if pat.len() == 5 && !pat.chars().all(|seg| mappings[9].contains(seg)) {
                mappings[2] = pat.to_string();
                false
            } else {
                true
            }
        });

        // 5: it’s not 2 nor 3
        patterns.retain(|pat| {
            if pat.len() == 5 {
                mappings[5] = pat.to_string();
                false
            } else {
                true
            }
        });

        // 6: contains 5 and is not 9
        patterns.retain(|pat| {
            if pat.len() == 6
                && pat != &mappings[9]
                && mappings[5].chars().all(|seg| pat.contains(seg))
            {
                mappings[6] = pat.to_string();
                false
            } else {
                true
            }
        });

        // 0 is the last one we haven’t found yet
        mappings[0] = patterns[0].to_string();

        // lol we’re done, let’s decode those shitty numbers; first, reverse the mappings so that we can work correctly
        let digits: HashMap<_, _> = mappings
            .into_iter()
            .enumerate()
            .map(|(i, digit)| (sort_string(&digit), i as u32))
            .collect();

        let n = output
            .into_iter()
            .flat_map(|out| digits.get(&sort_string(out)))
            .fold(0, |n, d| n * 10 + d);

        sum += n;
    }
    println!("Part two: {}", sum);
}

fn sort_string(s: &str) -> String {
    let mut bytes: Vec<_> = s.bytes().collect();
    bytes.sort();
    unsafe { String::from_utf8_unchecked(bytes) }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_lines = utils::read_file(&args[1]);
    part_one(&file_lines);
    part_two();
}

#[test]
fn parse_line_correctly() {
    let input = "acedgfb cdfbe ab | cdfeb cdbaf";
    assert_eq!(
        parse_line(input),
        (
            vec![
                Pattern {
                    text: String::from("abcdefg"),
                    value: Some(8)
                },
                Pattern {
                    text: String::from("bcdef"),
                    value: None
                },
                Pattern {
                    text: String::from("ab"),
                    value: Some(1)
                }
            ],
            vec!["bcdef".to_string(), "abcdf".to_string()]
        )
    );
}
