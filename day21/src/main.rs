use std::collections::HashMap;

struct Dice {
    last_roll: u128,
    rolls: u128,
}

impl Dice {
    fn new() -> Self {
        Dice {
            last_roll: 100,
            rolls: 0,
        }
    }

    fn next(&mut self) -> u128 {
        let next = if self.last_roll == 100 {
            1
        } else {
            self.last_roll + 1
        };
        self.last_roll = next;
        self.rolls += 1;
        next
    }
}

fn move_places(place: &mut u128, places: u128) {
    *place += places;
    while *place > 10 {
        *place -= 10;
    }
}
fn part_one(lines: &[String]) -> u128 {
    let mut p1_place = sscanf::scanf!(lines[0], "Player 1 starting position: {}", u128).unwrap();
    let mut p2_place = sscanf::scanf!(lines[1], "Player 2 starting position: {}", u128).unwrap();

    let mut dice = Dice::new();
    let mut p1_score = 0u128;
    let mut p2_score = 0u128;

    loop {
        // p1 go
        move_places(&mut p1_place, dice.next() + dice.next() + dice.next());
        p1_score += p1_place;
        if p1_score >= 1000 {
            return p2_score * dice.rolls;
        }

        // p2 go
        move_places(&mut p2_place, dice.next() + dice.next() + dice.next());
        p2_score += p2_place;
        if p2_score >= 1000 {
            return p1_score * dice.rolls;
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct GameState {
    p1_score: u128,
    p2_score: u128,
    p1_place: u128,
    p2_place: u128,
}

static TRIPLE_ROLL_RESULTS: &[(u128, u128)] =
    &[(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
fn modulo(mut value: u128, min: u128, max: u128) -> u128 {
    if value >= max {
        let range = max - min;
        let q = 1 + (value - max) / range;
        value -= q * range;
    }

    value
}

fn solve_recursive(state: GameState, cache: &mut HashMap<GameState, (u128, u128)>) -> (u128, u128) {
    if cache.contains_key(&state) {
        return *cache.get(&state).unwrap();
    }

    if state.p1_score >= 21 {
        return (1, 0);
    }
    if state.p2_score >= 21 {
        return (0, 1);
    }

    let mut total_p1_wins = 0;
    let mut total_p2_wins = 0;

    for (roll, freq) in TRIPLE_ROLL_RESULTS {
        let new_position = modulo(state.p1_place + roll, 1, 11);
        let new_score = state.p1_score + new_position;

        let (p2_wins, p1_wins) = solve_recursive(
            GameState {
                p1_score: state.p2_score,
                p2_score: new_score,
                p1_place: state.p2_place,
                p2_place: new_position,
            },
            cache,
        );
        total_p1_wins += freq * p1_wins;
        total_p2_wins += freq * p2_wins;
    }

    cache.insert(state, (total_p1_wins, total_p2_wins));

    (total_p1_wins, total_p2_wins)
}

fn part_two(lines: &[String]) -> u128 {
    let p1_place = sscanf::scanf!(lines[0], "Player 1 starting position: {}", u128).unwrap();
    let p2_place = sscanf::scanf!(lines[1], "Player 2 starting position: {}", u128).unwrap();
    let (p1_wins, p2_wins) = solve_recursive(
        GameState {
            p1_score: 0,
            p2_score: 0,
            p1_place,
            p2_place,
        },
        &mut HashMap::new(),
    );
    if p1_wins > p2_wins {
        p1_wins
    } else {
        p2_wins
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let lines = utils::read_file(&args[1]);

    println!("Part one: {}", part_one(&lines));
    println!("Part two: {}", part_two(&lines));
}
