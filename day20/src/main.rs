use std::fmt::Display;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let lines = utils::read_file(&args[1]);
    let map = Map::new(&lines);
    println!("Part One: {}", simulate_enhancements(map.clone(), 2));
    println!("Part Two: {}", simulate_enhancements(map, 50));
}

fn simulate_enhancements(map: Map, qty: usize) -> u32 {
    let mut map = map;
    for _ in 0..qty {
        map.enhance();
    }
    map.count_lights()
}

// Lit = true
// Dark = false

#[derive(Clone)]
struct Map {
    algorithm: [bool; 512],
    values: Vec<Vec<bool>>,
    outside_value: bool,
}

impl Map {
    fn new(lines: &[String]) -> Self {
        let algorithm: [bool; 512] = lines[0]
            .chars()
            .map(|c| c == '#')
            .collect::<Vec<bool>>()
            .try_into()
            .unwrap();
        let mut values: Vec<Vec<bool>> = vec![];
        for line in lines.iter().skip(2) {
            values.push(line.chars().map(|c| c == '#').collect());
        }
        Map {
            algorithm,
            values,
            outside_value: false,
        }
    }

    fn get_point(&self, x: isize, y: isize) -> bool {
        let width: isize = self.values[0].len().try_into().unwrap();
        let height: isize = self.values.len().try_into().unwrap();
        if x < 0 || x >= width || y < 0 || y >= height {
            return self.outside_value;
        }
        self.values[y as usize][x as usize]
    }

    fn get_new_point_value(&self, x: isize, y: isize) -> bool {
        let mut binary: Vec<bool> = vec![];
        for y in y - 1..y + 2 {
            for x in x - 1..x + 2 {
                binary.push(self.get_point(x, y));
            }
        }
        let number = utils::binary_to_decimal(binary);
        // println!("number for {},{}: {}", x, y, number);
        self.algorithm[number as usize]
    }

    fn enhance(&mut self) {
        let mut new_values: Vec<Vec<bool>> = vec![];
        let width: isize = self.values[0].len().try_into().unwrap();
        let height: isize = self.values.len().try_into().unwrap();

        for y in 0..height + 2 {
            new_values.push(vec![]);
            for x in 0..width + 2 {
                new_values[y as usize].push(self.get_new_point_value(x - 1, y - 1));
            }
        }
        self.values = new_values;
        match self.outside_value {
            true => self.outside_value = self.algorithm[511],
            false => self.outside_value = self.algorithm[0],
        };
    }

    fn count_lights(&self) -> u32 {
        let mut acc = 0u32;
        for row in &self.values {
            for val in row {
                if *val {
                    acc += 1
                }
            }
        }
        acc
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.values {
            for value in row {
                write!(f, "{}", if *value { '#' } else { '.' })?
            }
            writeln!(f)?
        }
        writeln!(
            f,
            "Outside is {}",
            if self.outside_value { "light" } else { "dark" }
        )?;
        writeln!(f, "Lights: {}", self.count_lights())?;
        Ok(())
    }
}
