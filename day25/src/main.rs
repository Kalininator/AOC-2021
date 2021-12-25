use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Space {
    Empty,
    SeaCucumber(Direction),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right,
    Down,
}

#[derive(Debug)]
struct ParseSpaceError;

impl Space {
    fn from_char(c: &char) -> Result<Self, ParseSpaceError> {
        match c {
            '.' => Ok(Space::Empty),
            '>' => Ok(Space::SeaCucumber(Direction::Right)),
            'v' => Ok(Space::SeaCucumber(Direction::Down)),
            _ => Err(ParseSpaceError),
        }
    }
}

struct Map {
    rows: Vec<Vec<Space>>,
}

impl Map {
    fn from_lines(lines: &[String]) -> Result<Self, ParseSpaceError> {
        Ok(Map {
            rows: lines
                .iter()
                .map(|l| l.chars().map(|c| Space::from_char(&c).unwrap()).collect())
                .collect(),
        })
    }

    fn print(&self) {
        for r in &self.rows {
            for space in r {
                print!(
                    "{}",
                    match space {
                        Space::Empty => '.',
                        Space::SeaCucumber(Direction::Right) => '>',
                        Space::SeaCucumber(Direction::Down) => 'v',
                    }
                );
            }
            println!();
        }
        println!();
    }

    fn get(&self, column: isize, row: isize) -> &Space {
        let width = self.rows[0].len();
        let height = self.rows.len();
        &self.rows[to_index(row, height - 1)][to_index(column, width - 1)]
    }

    fn next_step(&mut self) -> bool {
        let width = self.rows[0].len();
        let height = self.rows.len();
        let mut did_move = false;

        let mut new_rows = self.rows.clone();
        for y in 0..height {
            for x in 0..width {
                if self.rows[y][x] == Space::SeaCucumber(Direction::Right)
                    && self.rows[y][(x + 1) % width] == Space::Empty
                {
                    // println!("Move {},{} right to {},{}", y, x, y, (x + 1) % width);
                    new_rows[y][(x + 1) % width] = Space::SeaCucumber(Direction::Right);
                    new_rows[y][x] = Space::Empty;
                    did_move = true;
                }
            }
        }

        let mut new_new_rows = new_rows.clone();
        for y in 0..height {
            for x in 0..width {
                if new_rows[y][x] == Space::SeaCucumber(Direction::Down)
                    && new_rows[(y + 1) % height][x] == Space::Empty
                {
                    // println!("Move {},{} down to {},{}", y, x, (y + 1) % height, x);
                    new_new_rows[(y + 1) % height][x] = Space::SeaCucumber(Direction::Down);
                    new_new_rows[y][x] = Space::Empty;
                    did_move = true;
                }
            }
        }
        self.rows = new_new_rows;
        did_move
    }
}

fn to_index(i: isize, max: usize) -> usize {
    if i < 0 {
        max - i.abs() as usize
    } else if i as usize > max {
        (i as usize) - (max + 1)
    } else {
        i as usize
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let lines = utils::read_file(&args[1]);
    let mut map = Map::from_lines(&lines).unwrap();
    map.print();
    // map.print();
    // map.next_step();
    // map.print();
    // map.next_step();
    // map.print();
    // map.next_step();
    // map.print();
    let mut steps = 1usize;
    while map.next_step() {
        steps += 1;
        // println!("Step {}", steps);
    }
    println!("Steps: {}", steps);
    // for _ in 0..4 {
    //     map.next_step();
    //     map.print();
    // }
    // loop {
    //     let moved = map.next_step();
    //     println!("Moved: {:?}", moved);
    // }
}

#[test]
fn to_index_test() {
    assert_eq!(to_index(5, 4), 0);
}
