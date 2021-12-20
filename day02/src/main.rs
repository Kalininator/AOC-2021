fn parse_steps(lines: &[String]) -> i32 {
    let mut position: i32 = 0;
    let mut depth: i32 = 0;
    for line in lines {
        let mut spl = line.split(' ');
        let direction = spl.next().unwrap();
        let amount: i32 = spl.next().unwrap().parse().unwrap();
        match direction {
            "down" => depth += amount,
            "up" => depth -= amount,
            "forward" => position += amount,
            _ => (),
        }
    }
    position * depth
}

fn parse_steps_2(lines: &[String]) -> i32 {
    let mut aim: i32 = 0;
    let mut depth: i32 = 0;
    let mut position: i32 = 0;
    for line in lines {
        let mut spl = line.split(' ');
        let direction = spl.next().unwrap();
        let amount: i32 = spl.next().unwrap().parse().unwrap();
        match direction {
            "down" => {
                aim += amount;
            }
            "up" => {
                aim -= amount;
            }
            "forward" => {
                position += amount;
                depth += aim * amount;
            }
            _ => (),
        }
    }
    position * depth
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let lines = utils::read_file(&args[1]);
    let part_1_result = parse_steps(&lines);
    println!("Part 1 result: {}", part_1_result);
    let part_2_result = parse_steps_2(&lines);
    println!("Part 2 result: {}", part_2_result);
}
