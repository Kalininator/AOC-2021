use fxhash::FxHashSet;
use itertools::Itertools;

type Point = [i32; 3];

struct BeaconMap {
    readings: Vec<ScanResults>,
    beacon_locations: FxHashSet<Point>,
    beacon_distance_sets: Vec<FxHashSet<usize>>,
    total_distances: Vec<Point>,
}

impl BeaconMap {
    fn merge_all_readings(&mut self) {
        let mut total_distances = Vec::with_capacity(self.readings.len() + 1);
        total_distances.push([0, 0, 0]);
        while !self.readings.is_empty() {
            for i in (0..self.readings.len()).rev() {
                if let Some(d) = BeaconMap::merge_reading(
                    &mut self.beacon_locations,
                    &mut self.beacon_distance_sets,
                    &self.readings[i],
                ) {
                    total_distances.push(d);
                    self.readings.swap_remove(i);
                }
            }
        }
        self.total_distances.extend(total_distances);
    }

    fn merge_reading(
        beacon_locations: &mut FxHashSet<Point>,
        beacon_distance_sets: &mut Vec<FxHashSet<usize>>,
        reading: &ScanResults,
    ) -> Option<Point> {
        for rotation in 0..24 {
            let rotated = reading
                .beacons
                .iter()
                .map(|&beacon_location| get_orientation(beacon_location, rotation))
                .collect::<Vec<Point>>();
            let distances = beacon_locations
                .iter()
                .cartesian_product(&rotated)
                .map(|([x1, y1, z1], [x2, y2, z2])| [x1 - x2, y1 - y2, z1 - z2]);
            for [dx, dy, dz] in distances {
                let altered_rotated = rotated.iter().map(|[x, y, z]| [x + dx, y + dy, z + dz]);
                if altered_rotated
                    .clone()
                    .filter(|v| beacon_locations.contains(v))
                    .count()
                    >= 12
                {
                    beacon_locations.extend(altered_rotated);
                    beacon_distance_sets.push(reading.beacon_distances.clone());
                    return Some([dx, dy, dz]);
                }
            }
        }
        None
    }
}

struct ScanResults {
    beacons: Vec<Point>,
    beacon_distances: FxHashSet<usize>,
}

fn get_orientation([x, y, z]: Point, rot: u8) -> Point {
    match rot {
        0 => [x, y, z],
        1 => [x, z, -y],
        2 => [x, -y, -z],
        3 => [x, -z, y],
        4 => [y, x, -z],
        5 => [y, z, x],
        6 => [y, -x, z],
        7 => [y, -z, -x],
        8 => [z, x, y],
        9 => [z, y, -x],
        10 => [z, -x, -y],
        11 => [z, -y, x],
        12 => [-x, y, -z],
        13 => [-x, z, y],
        14 => [-x, -y, z],
        15 => [-x, -z, -y],
        16 => [-y, x, z],
        17 => [-y, z, -x],
        18 => [-y, -x, -z],
        19 => [-y, -z, x],
        20 => [-z, x, -y],
        21 => [-z, y, x],
        22 => [-z, -x, y],
        23 => [-z, -y, -x],
        _ => unreachable!(),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let lines = utils::read_file(&args[1]);
    let mut plane = parse(&lines);
    plane.merge_all_readings();
    let p1 = part1(&plane);
    println!("Part 1: {}", p1);
    let p2 = part2(&plane);
    println!("Part 2: {}", p2);
}

fn parse(lines: &[String]) -> BeaconMap {
    let mut readings = lines
        .join("\n")
        .split("\n\n")
        .map(|s| {
            let readings = s
                .lines()
                .skip(1)
                .map(|l| {
                    let mut parts = l.split(',');
                    let x = parts.next().unwrap().parse::<i32>().unwrap();
                    let y = parts.next().unwrap().parse::<i32>().unwrap();
                    let z = parts.next().unwrap().parse::<i32>().unwrap();
                    [x, y, z]
                })
                .collect::<Vec<_>>();
            let beacon_distance_set = readings
                .iter()
                .tuple_combinations()
                .map(|([x1, y1, z1], [x2, y2, z2])| {
                    ((x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()) as usize
                })
                .collect::<FxHashSet<_>>();
            ScanResults {
                beacons: readings,
                beacon_distances: beacon_distance_set,
            }
        })
        .collect::<Vec<_>>();
    let first_scanner = readings.remove(0);
    let beacon_locations = first_scanner.beacons.into_iter().collect::<FxHashSet<_>>();
    let total_distances = Vec::with_capacity(readings.len());
    let mut beacon_distance_sets = Vec::with_capacity(readings.len());
    beacon_distance_sets.push(first_scanner.beacon_distances);
    BeaconMap {
        readings,
        beacon_locations,
        beacon_distance_sets,
        total_distances,
    }
}

fn part1(plane: &BeaconMap) -> usize {
    plane.beacon_locations.len()
}

fn part2(plane: &BeaconMap) -> usize {
    plane
        .total_distances
        .iter()
        .tuple_combinations()
        .map(|([x1, y1, z1], [x2, y2, z2])| (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs())
        .max()
        .unwrap() as usize
}
