fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut lines = utils::read_file(&args[1]);
    // first line is instructions
    let instructions: Vec<u32> = lines
        .first()
        .unwrap()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect::<Vec<u32>>();
    // second line is blank
    lines.drain(0..2);

    let mut grids: Vec<Vec<Vec<Option<u32>>>> = lines
        .split(|l| l.is_empty())
        .map(|group| parse_grid(group))
        .collect();

    let mut grids_won: Vec<usize> = vec![];

    for drawn_number in instructions {
        for i in 0..grids.len() {
            clear_grid_value(&mut grids[i], drawn_number);
            if !grids_won.contains(&i) {
                let has_won = has_grid_won(&grids[i]);
                if has_won {
                    println!("Grid {} has won", i);
                    println!("Grid total {}", sum_board(&grids[i]));
                    println!("Final score {}", sum_board(&grids[i]) * drawn_number);
                    grids_won.push(i);
                }
            }
        }
    }
}

fn parse_grid(lines: &[String]) -> Vec<Vec<Option<u32>>> {
    lines
        .iter()
        .map(|l| {
            l.split_whitespace()
                .map(|n| Some(n.parse().unwrap()))
                .collect::<Vec<Option<u32>>>()
        })
        .collect()
}

fn clear_grid_value(grid: &mut Vec<Vec<Option<u32>>>, value: u32) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == Some(value) {
                grid[i][j] = None;
            }
        }
    }
}

fn has_grid_won(grid: &Vec<Vec<Option<u32>>>) -> bool {
    // check rows
    for row in 0..grid.len() {
        if grid[row].iter().all(|x| x.is_none()) {
            return true;
        }
    }

    for column in 0..grid[0].len() {
        let mut has_some = false;
        for row in 0..grid.len() {
            if grid[row][column].is_some() {
                has_some = true;
            }
        }
        if !has_some {
            return true;
        }
    }

    false
}

fn sum_board(grid: &Vec<Vec<Option<u32>>>) -> u32 {
    let mut acc: u32 = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if let Some(val) = grid[i][j] {
                acc += val;
            }
        }
    }
    acc
}
