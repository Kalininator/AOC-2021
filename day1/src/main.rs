use utils;

// each line is sear floor depth
//
// count number of times depth increases

fn count_increases(depths: &[u32]) -> usize {
    let mut increases: usize = 0;
    for i in 1..depths.len() {
        if depths[i] > depths[i - 1] {
            increases += 1;
        }
    }
    increases
}

fn get_window_totals(depths: &[u32]) -> Vec<u32> {
    let mut totals: Vec<u32> = vec![];

    for i in 2..depths.len() {
        let total = depths[i - 2] + depths[i - 1] + depths[i];
        totals.push(total);
    }

    totals
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let lines = utils::read_file(&args[1]);
    let depths: Vec<u32> = lines.iter().map(|l| l.parse().unwrap()).collect();
    println!("Part 1 Depth increases: {}", count_increases(&depths));

    let windows = get_window_totals(&depths);
    println!("Part 2 Depth increases: {}", count_increases(&windows));
}

#[test]
fn count_increases_test() {
    assert_eq!(
        count_increases(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
        7
    );
}

#[test]
fn get_window_totals_test() {
    assert_eq!(
        get_window_totals(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
        vec![607, 618, 618, 617, 647, 716, 769, 792]
    );
}
