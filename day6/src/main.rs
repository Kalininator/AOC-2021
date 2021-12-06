// each lanternfish created a new lanternfish every 7 days
// each fish is a single number that represents days until new fish
// new fish need extra 2 days to create a fish

type Fish = u8;

fn next_day(fish_at_days: &mut Vec<u128>) {
    let fish_to_reproduce = fish_at_days[0];
    fish_at_days.drain(0..1);
    fish_at_days.push(fish_to_reproduce);
    fish_at_days[6] += fish_to_reproduce;
}

fn simulate_days(fish: &[Fish], days: u32) -> u128 {
    let collection = fish.to_owned();
    let mut fish_at_days = parse_fish_to_days(collection);
    for _ in 0..days {
        next_day(&mut fish_at_days);
    }
    fish_at_days.iter().sum()
}

fn parse_fish_to_days(fish: Vec<Fish>) -> Vec<u128> {
    let mut fish_at_days: Vec<u128> = vec![0; 9];
    for f in fish {
        fish_at_days[f as usize] += 1;
    }
    fish_at_days
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_lines = utils::read_file(&args[1]);
    let fish: Vec<Fish> = file_lines
        .first()
        .unwrap()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();
    println!("Fish after 80 days: {}", simulate_days(&fish, 80));
    println!("Fish after 256 days: {}", simulate_days(&fish, 256));
}

#[test]
fn simulate_days_test() {
    let fish: Vec<Fish> = vec![3, 4, 3, 1, 2];
    let count = simulate_days(&fish, 80);
    assert_eq!(count, 5934);
}
