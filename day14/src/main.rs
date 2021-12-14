use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut lines = utils::read_file(&args[1]);

    let mut chars: Vec<char> = lines
        .first()
        .expect("First line not found")
        .chars()
        .collect();

    // First line is read above, second line is blank
    lines.drain(0..2);

    let mut insertion_rules: HashMap<String, char> = HashMap::new();

    for line in lines {
        let line_sections: Vec<&str> = line.split_whitespace().collect();
        insertion_rules.insert(
            line_sections[0].to_string(),
            line_sections[2].chars().collect::<Vec<char>>()[0],
        );
    }

    print_chars(&chars);
    for i in 0..40 {
        println!("Running step: {}", i + 1);
        step(&mut chars, &insertion_rules);
        if i == 9 {
            println!("Part 1 result: {}", count_score(&chars));
        }
    }
    println!("Part 2 result: {}", count_score(&chars));
}

fn print_chars(chars: &[char]) {
    println!("{}", chars.iter().collect::<String>());
}

fn count_score(chars: &[char]) -> u128 {
    let mut map: HashMap<char, u128> = HashMap::new();
    for c in chars {
        let entry = map.entry(*c).or_insert(0);
        *entry += 1;
    }
    map.values().max().unwrap() - map.values().min().unwrap()
}

fn step(chars: &mut Vec<char>, insertion_rules: &HashMap<String, char>) {
    let pairs: Vec<String> = chars.windows(2).map(|w| w.iter().collect()).collect();
    let vals_to_insert: Vec<Option<&char>> = pairs
        .iter()
        .map(|pair| insertion_rules.get(pair))
        .collect::<Vec<Option<&char>>>();
    let existing_values: Vec<char> = chars.drain(..).collect();
    for i in 0..vals_to_insert.len() {
        chars.push(existing_values[i]);
        if let Some(val) = vals_to_insert[i] {
            chars.push(*val);
        }
    }
    chars.push(*existing_values.last().unwrap());
}
