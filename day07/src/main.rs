fn part_one(crabs: &[i32]) -> i32 {
    let mut sorted_crabs = crabs.to_owned();
    sorted_crabs.sort_unstable();
    let average_position = sorted_crabs[crabs.len() / 2];
    sorted_crabs
        .iter()
        .map(|c| (c - average_position).abs())
        .sum()
}

fn part_two(crabs: &[i32]) -> i32 {
    let mut sorted_crabs = crabs.to_owned();
    sorted_crabs.sort_unstable();
    let mut costs: Vec<i32> = vec![];
    let min = *sorted_crabs.first().unwrap();
    let max = *sorted_crabs.last().unwrap();
    for position in min..=max {
        costs.push(
            crabs
                .iter()
                .map(|crab| calculate_cost((position - crab).abs()))
                .sum(),
        );
    }
    *costs.iter().min().unwrap()
}

fn calculate_cost(distance: i32) -> i32 {
    let mut acc: i32 = 0;
    for i in 0..=distance {
        acc += i;
    }
    acc
}

fn parse_crabs(line: &str) -> Vec<i32> {
    line.split(',').map(|c| c.parse().unwrap()).collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_lines = utils::read_file(&args[1]);
    let crabs = parse_crabs(file_lines.first().unwrap());
    println!("Part one result: {}", part_one(&crabs));
    println!("Part two result: {}", part_two(&crabs));
}

#[test]
fn part_one_test() {
    let crabs = parse_crabs(&String::from("16,1,2,0,4,2,7,1,2,14"));
    assert_eq!(part_one(&crabs), 37);
}

#[test]
fn part_two_test() {
    let crabs = parse_crabs(&String::from("16,1,2,0,4,2,7,1,2,14"));
    assert_eq!(part_two(&crabs), 168);
}
