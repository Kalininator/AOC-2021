fn main() {
    let args: Vec<String> = std::env::args().collect();
    let lines = utils::read_file(&args[1]);
    let parsed = sscanf::scanf!(
        lines[0],
        "target area: x={}..{}, y={}..{}",
        i32,
        i32,
        i32,
        i32
    )
    .unwrap();
    println!("Target: {:?}", parsed);

    let mut successfull_velocities: Vec<(i32, i32, i32)> = vec![];

    for x in 0..1000 {
        for y in -1000..1000 {
            if let VelocityResult::Success(highest) = try_velocity(&(x, y), parsed) {
                successfull_velocities.push((x, y, highest));
            }
        }
    }

    // 6,0 7,-1 should show but dont

    try_velocity(&(6, 0), parsed);
    try_velocity(&(7, -1), parsed);

    println!("Velocities: {:?}", successfull_velocities);
    for v in &successfull_velocities {
        println!("{},{}", v.0, v.1);
    }

    let highest_y = successfull_velocities.iter().map(|v| v.2).max().unwrap();
    println!("Highest Y: {}", highest_y);
    println!("Successful count: {}", successfull_velocities.len());
    //
    //     println!("{:?}", try_velocity(&velocity, parsed));
}

#[derive(Debug)]
enum VelocityResult {
    Success(i32),
    Fail,
}

fn try_velocity(velocity: &(i32, i32), target: (i32, i32, i32, i32)) -> VelocityResult {
    let mut position = (0i32, 0i32);
    let mut velocity = *velocity;

    let mut highest_y = 0i32;

    loop {
        // Step
        position.0 += velocity.0;
        position.1 += velocity.1;
        // println!("Moved to {},{}", position.0, position.1);
        if velocity.0 > 0 {
            velocity.0 -= 1;
        } else if velocity.0 < -0 {
            velocity.0 += 1;
        }
        velocity.1 -= 1;
        if position.1 > highest_y {
            highest_y = position.1
        }

        // Check if in target area
        if position.0 <= target.1
            && position.0 >= target.0
            && position.1 >= target.2
            && position.1 <= target.3
        {
            // println!("Landed in target at {},{}", position.0, position.1);
            return VelocityResult::Success(highest_y);
        }
        if position.0 > target.1 || position.1 < target.2 {
            // println!("Out of bounds");
            return VelocityResult::Fail;
        }
    }
}
