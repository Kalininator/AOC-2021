fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_lines = utils::read_file(&args[1]);
    let mut grid = Grid::from_lines(&file_lines);
    part_one(&mut grid);
    let mut grid = Grid::from_lines(&file_lines);
    part_two(&mut grid);
}

fn part_one(grid: &mut Grid) {
    let mut acc: u32 = 0;
    for _ in 0..100 {
        acc += grid.step();
    }
    println!("Part One {}", acc);
}

fn part_two(grid: &mut Grid) {
    let mut acc: u32 = 0;
    loop {
        acc += 1;
        let flashes = grid.step();
        if flashes == 100 {
            println!("All flashed on step {}", acc);
            break;
        }
    }
}

fn minus(val: usize, min: usize) -> usize {
    if val == min {
        min
    } else {
        val - 1
    }
}

fn plus(val: usize, max: usize) -> usize {
    if val == max {
        max
    } else {
        val + 1
    }
}

#[derive(Debug)]
struct Octopus {
    value: u32,
    flashed: bool,
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<Octopus>>,
}

impl Grid {
    fn from_lines(lines: &[String]) -> Self {
        Grid {
            grid: lines
                .iter()
                .map(|l| {
                    l.chars()
                        .map(|c| Octopus {
                            value: c.to_digit(10).unwrap(),
                            flashed: false,
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn increment_all(&mut self) {
        for row in 0..self.grid.len() {
            for column in 0..self.grid[0].len() {
                self.grid[row][column].value += 1;
            }
        }
    }

    fn trigger_flashes(&mut self) -> bool {
        let mut have_flashed = false;
        let height = self.grid.len();
        let width = self.grid[0].len();
        for row in 0..height {
            for column in 0..width {
                if self.grid[row][column].value > 9 && !self.grid[row][column].flashed {
                    self.grid[row][column].flashed = true;
                    have_flashed = true;
                    for i in minus(row, 0)..=plus(row, height - 1) {
                        for j in minus(column, 0)..=plus(column, width - 1) {
                            if !(i == row && j == column) {
                                self.grid[i][j].value += 1;
                            }
                        }
                    }
                }
            }
        }
        have_flashed
    }

    fn reset_flashes(&mut self) -> u32 {
        let mut flashes: u32 = 0;
        for row in 0..self.grid.len() {
            for column in 0..self.grid[0].len() {
                if self.grid[row][column].flashed {
                    flashes += 1;
                    self.grid[row][column].flashed = false;
                    self.grid[row][column].value = 0;
                }
            }
        }
        flashes
    }

    fn step(&mut self) -> u32 {
        self.increment_all();
        while self.trigger_flashes() {}
        self.reset_flashes()
    }
}

#[test]
fn part_one_example() {
    let lines = vec![
        String::from("11111"),
        String::from("19991"),
        String::from("19191"),
        String::from("19991"),
        String::from("11111"),
    ];
    let mut grid = Grid::from_lines(&lines);
    assert_eq!(grid.step(), 9);
}
