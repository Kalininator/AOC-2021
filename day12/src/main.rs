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

    println!("{:?}", map);

    let mut paths = get_all_paths(&map);
    paths.retain(|p| p.last() == Some(&Cave::End));

    println!("Paths to end: {}", paths.len());
    //
    //     for path in paths {
    //         println!("Path: {:?}", path);
    //     }
}

fn get_all_paths(map: &HashMap<Cave, Vec<Cave>>) -> Vec<Vec<Cave>> {
    get_paths_to_end(map, Cave::Start, vec![])
}

fn get_paths_to_end(map: &HashMap<Cave, Vec<Cave>>, from: Cave, path: Vec<Cave>) -> Vec<Vec<Cave>> {
    // println!("Path {:?} moving to {:?}", path, from);
    // If reached end then we good
    if let Cave::End = from {
        return vec![path];
    }
    let mut links = map.get(&from).unwrap_or(&vec![]).clone();
    // remove small caves that have been visited
    links.retain(|link| match link {
        Cave::Small(cave) => !path.contains(&Cave::Small(cave.clone())),
        Cave::Start => false,
        _ => true,
    });
    if links.is_empty() {
        return vec![path];
    }
    links
        .iter()
        .map(|l| get_paths_to_end(map, l.clone(), [path.clone(), vec![l.clone()]].concat()))
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
