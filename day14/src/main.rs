use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut lines = utils::read_file(&args[1]);

    let chars: Vec<char> = lines
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

    let mut pair_map: HashMap<String, u128> = HashMap::new();

    for pair_string in chars.windows(2) {
        let entry = pair_map.entry(pair_string.iter().collect()).or_insert(0);
        *entry += 1;
    }

    print_chars(&chars);
    println!("{:?}", pair_map);
    for _i in 0..10 {
        step_pairs(&mut pair_map, &insertion_rules);
    }

    println!("Part 1 result: {}", count_from_map(&chars, &pair_map));
    for _i in 0..30 {
        step_pairs(&mut pair_map, &insertion_rules);
    }
    println!("Part 2 result: {}", count_from_map(&chars, &pair_map));
}

fn print_chars(chars: &[char]) {
    println!("{}", chars.iter().collect::<String>());
}

fn count_from_map(original_formula: &[char], map: &HashMap<String, u128>) -> u128 {
    let mut count: HashMap<char, u128> = HashMap::new();
    for pair in map {
        if *pair.1 == 0 {
            continue;
        }
        for c in pair.0.chars() {
            let entry = count.entry(c).or_insert(0);
            *entry += pair.1;
        }
    }
    *(count.entry(*original_formula.last().unwrap()).or_insert(0)) += 1;
    (count.values().max().unwrap() - count.values().min().unwrap()) / 2
}

fn step_pairs(pairs: &mut HashMap<String, u128>, insertion_rules: &HashMap<String, char>) {
    let mut new_pairs: HashMap<String, u128> = HashMap::new();
    for pair in pairs.iter_mut() {
        let new_char = insertion_rules.get(pair.0);
        if let Some(c) = new_char {
            let count = *pair.1;
            *pair.1 = 0;
            let left_pair: String =
                vec![*pair.0.chars().collect::<Vec<char>>().get(0).unwrap(), *c]
                    .iter()
                    .collect();
            let right_pair: String =
                vec![*c, *pair.0.chars().collect::<Vec<char>>().get(1).unwrap()]
                    .iter()
                    .collect();
            let left_entry = new_pairs.entry(left_pair).or_insert(0);
            *left_entry += count;
            let right_entry = new_pairs.entry(right_pair).or_insert(0);
            *right_entry += count;
        }
    }
    for new_pair in new_pairs {
        let entry = pairs.entry(new_pair.0).or_insert(0);
        *entry += new_pair.1;
    }
}
