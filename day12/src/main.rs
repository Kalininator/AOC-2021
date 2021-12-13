use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_lines = utils::read_file(&args[1]);

    let mut map: HashMap<Cave, Vec<Cave>> = HashMap::new();

    for line in file_lines {
        let (left, right) = parse_line(&line);
        let left_entry = map.entry(left.clone()).or_insert_with(Vec::new);
        left_entry.push(right.clone());
        let right_entry = map.entry(right.clone()).or_insert_with(Vec::new);
        right_entry.push(left.clone());
    }
    part_1(&map);
    part_2(&map);
}

fn part_1(map: &HashMap<Cave, Vec<Cave>>) {
    let mut paths = get_all_paths(map, SmallCaveBehaviour::OnlyOnce);
    paths.retain(|p| p.last() == Some(&Cave::End));
    println!("Part 1: {}", paths.len());
}

fn part_2(map: &HashMap<Cave, Vec<Cave>>) {
    let mut paths = get_all_paths(map, SmallCaveBehaviour::OneCaveTwice);
    paths.retain(|p| p.last() == Some(&Cave::End));
    println!("Part 2: {}", paths.len());
}

#[derive(Clone, Copy)]
enum SmallCaveBehaviour {
    OnlyOnce,
    OneCaveTwice,
}

fn get_all_paths(
    map: &HashMap<Cave, Vec<Cave>>,
    small_cave_behaviour: SmallCaveBehaviour,
) -> Vec<Vec<Cave>> {
    get_paths_to_end(map, Cave::Start, vec![], small_cave_behaviour)
}

fn has_multiple_same_small_cave(path: &[Cave]) -> bool {
    let mut caves_hash = HashMap::new();
    for cave in path {
        if let Cave::Small(val) = cave {
            let entry = caves_hash.entry(val).or_insert(0);
            *entry += 1;
        }
    }
    caves_hash.values().any(|&x| x > 1)
}

fn can_visit_cave(path: &[Cave], cave: Cave, small_cave_behaviour: SmallCaveBehaviour) -> bool {
    match cave {
        // // Part 1
        // Cave::Small(_) => !path.contains(&cave),
        // Part 2
        Cave::Small(_) => {
            if let SmallCaveBehaviour::OneCaveTwice = small_cave_behaviour {
                return !path.contains(&cave) || !has_multiple_same_small_cave(path);
            }
            !path.contains(&cave)
        }
        Cave::Start => false,
        _ => true,
    }
}

fn get_paths_to_end(
    map: &HashMap<Cave, Vec<Cave>>,
    from: Cave,
    path: Vec<Cave>,
    small_cave_behaviour: SmallCaveBehaviour,
) -> Vec<Vec<Cave>> {
    // println!("Path {:?} moving to {:?}", path, from);
    // If reached end then we good
    if let Cave::End = from {
        return vec![path];
    }
    let mut links = map.get(&from).unwrap_or(&vec![]).clone();
    // remove small caves that have been visited
    links.retain(|link| can_visit_cave(&path, link.clone(), small_cave_behaviour));
    if links.is_empty() {
        return vec![path];
    }
    links
        .iter()
        .map(|l| {
            get_paths_to_end(
                map,
                l.clone(),
                [path.clone(), vec![l.clone()]].concat(),
                small_cave_behaviour,
            )
        })
        .collect::<Vec<Vec<Vec<Cave>>>>()
        .concat()
}

fn parse_line(line: &str) -> (Cave, Cave) {
    let sections: Vec<&str> = line.split('-').collect();
    (sections[0].parse().unwrap(), sections[1].parse().unwrap())
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Cave {
    Start,
    End,
    Small(String),
    Big(String),
}

#[derive(Debug)]
struct ParseCaveError;

impl FromStr for Cave {
    type Err = ParseCaveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let is_lowercase = s == s.to_lowercase();
        let is_uppercase = s == s.to_uppercase();
        match s {
            "start" => Ok(Cave::Start),
            "end" => Ok(Cave::End),
            _ => {
                if is_lowercase {
                    Ok(Cave::Small(s.to_string()))
                } else if is_uppercase {
                    Ok(Cave::Big(s.to_string()))
                } else {
                    Err(ParseCaveError)
                }
            }
        }
    }
}

#[test]
fn has_multiple_same_small_cave_test() {
    assert_eq!(
        has_multiple_same_small_cave(&[
            Cave::Small("abc".to_string()),
            Cave::Small("abc".to_string()),
            Cave::Small("foo".to_string())
        ]),
        true
    );
}
