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

fn simulate_days(fish: &Vec<Fish>, days: u32) -> usize {
    let mut collection = fish.clone();
    for i in 0..days {
        println!("Day {}", i);
        next_day(&mut collection);
    }
    collection.len()
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
