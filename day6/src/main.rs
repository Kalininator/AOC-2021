// each lanternfish created a new lanternfish every 7 days
// each fish is a single number that represents days until new fish
// new fish need extra 2 days to create a fish

type Fish = u8;

fn next_day(fish: &mut Vec<Fish>) {
    let mut new_fish: Vec<Fish> = vec![];
    for f in fish.iter_mut() {
        if *f == 0 {
            new_fish.push(8);
            *f = 6;
            // reproduce
        } else {
            *f -= 1;
        }
    }
    fish.append(&mut new_fish);
}

fn next_day_fast(fish_at_days: &mut [u128; 9]) {
    let fish_to_reproduce = fish_at_days[0];
    fish_at_days[0] = fish_at_days[1];
    fish_at_days[1] = fish_at_days[2];
    fish_at_days[2] = fish_at_days[3];
    fish_at_days[3] = fish_at_days[4];
    fish_at_days[4] = fish_at_days[5];
    fish_at_days[5] = fish_at_days[6];
    fish_at_days[6] = fish_at_days[7] + fish_to_reproduce;
    fish_at_days[7] = fish_at_days[8];
    fish_at_days[8] = fish_to_reproduce;
}

fn simulate_days(fish: &[Fish], days: u32) -> u128 {
    let collection = fish.to_owned();
    let mut fish_at_days: [u128; 9] = [0; 9];
    for fish in collection {
        fish_at_days[fish as usize] += 1;
    }
    for _ in 0..days {
        next_day_fast(&mut fish_at_days);
    }
    fish_at_days.iter().sum()
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
fn next_day_test() {
    let mut fish: Vec<Fish> = vec![3, 4, 3, 1, 2];
    for _ in 0..80 {
        next_day(&mut fish);
    }
    assert_eq!(fish.len(), 5934);
}
