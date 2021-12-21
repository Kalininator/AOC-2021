struct Dice {
    last_roll: u32,
    rolls: u32,
}

impl Dice {
    fn new() -> Self {
        Dice {
            last_roll: 100,
            rolls: 0,
        }
    }

    fn next(&mut self) -> u32 {
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

fn move_places(place: &mut u32, places: u32) {
    *place += places;
    while *place > 10 {
        *place -= 10;
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let lines = utils::read_file(&args[1]);
    let mut p1_place = sscanf::scanf!(lines[0], "Player 1 starting position: {}", u32).unwrap();
    let mut p2_place = sscanf::scanf!(lines[1], "Player 2 starting position: {}", u32).unwrap();

    let mut dice = Dice::new();
    let mut p1_score = 0u32;
    let mut p2_score = 0u32;

    loop {
        // p1 go
        move_places(&mut p1_place, dice.next() + dice.next() + dice.next());
        p1_score += p1_place;
        if p1_score >= 1000 {
            println!("P1 Won");
            println!("Part One: {}", p2_score * dice.rolls);
            break;
        }

        // p2 go
        move_places(&mut p2_place, dice.next() + dice.next() + dice.next());
        p2_score += p2_place;
        if p2_score >= 1000 {
            println!("P2 Won");
            println!("Part One: {}", p1_score * dice.rolls);
            break;
        }
    }
}
