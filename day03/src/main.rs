fn part_one(lines: &[String]) -> u32 {
    let length: usize = lines[0].len();

    let mut counts: Vec<u32> = vec![0; length];
    let line_count = lines.len();

    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        for i in 0..length {
            let val = chars[i].to_digit(10).unwrap();
            counts[i] += val;
        }
    }

    let mut gamma_rate_bools: Vec<bool> = vec![];
    let mut epsilon_rate_bools: Vec<bool> = vec![];

    for i in 0..length {
        if counts[i] < (line_count / 2).try_into().unwrap() {
            epsilon_rate_bools.push(true);
            gamma_rate_bools.push(false);
        } else {
            epsilon_rate_bools.push(false);
            gamma_rate_bools.push(true);
        }
    }
    binary_to_decimal(gamma_rate_bools) * binary_to_decimal(epsilon_rate_bools)
}

fn most_common_at_index(grid: &Vec<Vec<bool>>, index: usize) -> bool {
    let rows: u32 = grid.len().try_into().unwrap();
    println!("Rows: {}", rows);
    let mut acc: u32 = 0;
    for row in grid {
        acc += row[index] as u32;
    }
    println!("Acc: {}", acc);
    acc as f32 >= (rows as f32 / 2.0)
}

fn oxygen_rating(mut acc: Vec<bool>, mut grid: Vec<Vec<bool>>, index: usize) -> u32 {
    let most_common = most_common_at_index(&grid, index);
    println!("Most common: {}", most_common);
    acc.push(most_common);
    if index == grid[0].len() - 1 {
        binary_to_decimal(acc)
    } else {
        grid.retain(|row| row[index] == most_common);
        oxygen_rating(acc, grid, index + 1)
    }
}

fn least_common_at_index(grid: &Vec<Vec<bool>>, index: usize) -> bool {
    let rows: u32 = grid.len().try_into().unwrap();
    println!("Rows: {}", rows);
    let mut acc: u32 = 0;
    for row in grid {
        acc += row[index] as u32;
    }
    println!("Acc: {}", acc);
    (acc as f32) < (rows as f32 / 2.0)
}

fn scrubber_rating(mut acc: Vec<bool>, mut grid: Vec<Vec<bool>>, index: usize, len: usize) -> u32 {
    if grid.len() == 1 {
        return binary_to_decimal(grid[0].clone());
    }
    let least_common = least_common_at_index(&grid, index);
    println!("Least common: {}", least_common);
    acc.push(least_common);
    if index == len - 1 {
        binary_to_decimal(acc)
    } else {
        grid.retain(|row| row[index] == least_common);
        scrubber_rating(acc, grid, index + 1, len)
    }
}

fn part_two(lines: &[String]) -> u32 {
    let grid: Vec<Vec<bool>> = lines
        .iter()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() != 0).collect())
        .collect();

    let oxygen = oxygen_rating(vec![], grid.clone(), 0);
    println!("oxygen {}", oxygen);
    let len = grid[0].len();
    let scrubber = scrubber_rating(vec![], grid, 0, len);
    println!("scrubber {}", scrubber);
    oxygen * scrubber
}

fn binary_to_decimal(bools: Vec<bool>) -> u32 {
    println!("{:?}", bools);
    let mut acc: u32 = 0;
    for i in bools {
        match i {
            true => acc = (acc * 2) + 1,
            false => acc *= 2,
        }
    }
    println!("{}", acc);
    acc
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let lines = utils::read_file(&args[1]);
    let part_one_result = part_one(&lines);
    println!("Part one results: {}", part_one_result);
    let part_two_result = part_two(&lines);
    println!("Part two results: {}", part_two_result);
}

#[test]
fn part_one_test() {
    let input = vec![
        String::from("00100"),
        String::from("11110"),
        String::from("10110"),
        String::from("10111"),
        String::from("10101"),
        String::from("01111"),
        String::from("00111"),
        String::from("11100"),
        String::from("10000"),
        String::from("11001"),
        String::from("00010"),
        String::from("01010"),
    ];
    assert_eq!(part_one(&input), 198);
}

#[test]
fn part_two_test() {
    let input = vec![
        String::from("00100"),
        String::from("11110"),
        String::from("10110"),
        String::from("10111"),
        String::from("10101"),
        String::from("01111"),
        String::from("00111"),
        String::from("11100"),
        String::from("10000"),
        String::from("11001"),
        String::from("00010"),
        String::from("01010"),
    ];
    assert_eq!(part_two(&input), 230);
}
