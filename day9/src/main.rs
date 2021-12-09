use std::collections::HashSet;

struct Heightmap {
    map: Vec<Vec<u32>>,
    width: i32,
    height: i32,
}

impl Heightmap {
    fn new(lines: &[String]) -> Self {
        let map: Vec<Vec<u32>> = lines
            .iter()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        let width: i32 = map[0].len().try_into().unwrap();
        let height: i32 = map.len().try_into().unwrap();
        Heightmap { map, width, height }
    }

    fn in_bounds(&self, x: i32, y: i32) -> bool {
        (0..self.width).contains(&x) && (0..self.height).contains(&y)
    }

    fn get_point(&self, x: i32, y: i32) -> Option<u32> {
        match self.in_bounds(x, y) {
            true => Some(self.map[y as usize][x as usize]),
            false => None,
        }
    }

    fn is_low_point(&self, x: usize, y: usize) -> bool {
        let x = x as i32;
        let y = y as i32;
        // point is not in bounds
        if !self.in_bounds(x, y) {
            return false;
        }
        let val = self.get_point(x, y).unwrap();
        self.get_point(x - 1, y).unwrap_or(10) > val
            && self.get_point(x + 1, y).unwrap_or(10) > val
            && self.get_point(x, y - 1).unwrap_or(10) > val
            && self.get_point(x, y + 1).unwrap_or(10) > val
    }

    fn get_all_points(&self) -> Vec<(i32, i32)> {
        let mut points: Vec<(i32, i32)> = vec![];
        for x in 0..self.width {
            for y in 0..self.height {
                points.push((x, y));
            }
        }
        points
    }

    fn get_adjacent_points(&self, x: i32, y: i32) -> Vec<(i32, i32)> {
        let mut points: Vec<(i32, i32)> = vec![];
        if x > 0 {
            points.push((x - 1, y));
        }
        if x < self.width - 1 {
            points.push((x + 1, y));
        }
        if y > 0 {
            points.push((x, y - 1));
        }
        if y < self.height - 1 {
            points.push((x, y + 1));
        }
        points
    }

    fn get_basin_from_floor(&self, x: i32, y: i32) -> Vec<(i32, i32)> {
        let mut basin: Vec<(i32, i32)> = vec![];
        let val = self.get_point(x, y).unwrap();
        // add start point
        basin.push((x, y));
        let mut adjacents = self.get_adjacent_points(x, y);
        adjacents.retain(|p| {
            let v = self.get_point(p.0, p.1).unwrap();
            v > val && v < 9
        });
        for p in adjacents {
            basin.append(&mut self.get_basin_from_floor(p.0, p.1));
        }
        dedupe_points(basin)
    }
}

fn find_low_points(map: &Heightmap) -> Vec<u32> {
    let mut low_points: Vec<u32> = vec![];
    for x in 0..map.width {
        for y in 0..map.height {
            if map.is_low_point(x.try_into().unwrap(), y.try_into().unwrap()) {
                low_points.push(map.get_point(x, y).unwrap());
            }
        }
    }
    low_points
}

fn part_one(map: &Heightmap) -> u32 {
    find_low_points(map).iter().map(|i| i + 1).sum()
}

fn part_two(map: &Heightmap) -> usize {
    let mut points = map.get_all_points();
    // All points 9 or higher are not part of a basin
    points.retain(|p| map.get_point(p.0, p.1).unwrap() < 9);

    let mut basins: Vec<usize> = vec![];

    loop {
        let basin = map.get_basin_from_floor(points[0].0, points[0].1);
        points.retain(|p| !basin.contains(p));
        basins.push(basin.len());
        if points.is_empty() {
            break;
        }
    }

    basins.sort_unstable();
    basins.reverse();
    basins[0] * basins[1] * basins[2]
}

fn dedupe_points(points: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    for p in points {
        set.insert(p);
    }
    set.into_iter().collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_lines = utils::read_file(&args[1]);

    let heightmap = Heightmap::new(&file_lines);

    let p1 = part_one(&heightmap);
    println!("Part one result: {}", p1);
    let p2 = part_two(&heightmap);
    println!("Part two result: {}", p2);
}

#[test]
fn part_one_test() {
    let input: Vec<String> = vec![
        String::from("2199943210"),
        String::from("3987894921"),
        String::from("9856789892"),
        String::from("8767896789"),
        String::from("9899965678"),
    ];
    let map = Heightmap::new(&input);
    assert_eq!(part_one(&map), 15);
}

#[test]
fn part_two_test() {
    let input: Vec<String> = vec![
        String::from("2199943210"),
        String::from("3987894921"),
        String::from("9856789892"),
        String::from("8767896789"),
        String::from("9899965678"),
    ];
    let map = Heightmap::new(&input);
    assert_eq!(part_two(&map), 1134);
}
