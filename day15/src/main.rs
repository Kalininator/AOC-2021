use std::cmp::Reverse;
use std::collections::BinaryHeap;

type Point = (usize, usize);

fn get_adjacents(map: &[Vec<u32>], point: Point) -> Vec<Point> {
    let mut adjacents: Vec<Point> = vec![];
    let width = map[0].len();
    let height = map.len();
    if point.0 > 0 {
        adjacents.push((point.0 - 1, point.1));
    }
    if point.1 > 0 {
        adjacents.push((point.0, point.1 - 1));
    }
    if point.0 < width - 1 {
        adjacents.push((point.0 + 1, point.1));
    }
    if point.1 < height - 1 {
        adjacents.push((point.0, point.1 + 1));
    }
    adjacents
}

fn to_index(p: Point, l: usize) -> usize {
    p.1 * l + p.0
}

fn shortest_path(graph: &[Vec<u32>], start: Point, end: Point) -> u32 {
    let size = graph.len();

    let mut distance = vec![None; size * size];
    let mut predecessors = vec![None; size * size];
    distance[to_index(start, size)] = Some(0);

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, start)));

    while let Some(Reverse((dist, pos))) = heap.pop() {
        for neighbour in get_adjacents(graph, pos) {
            let neighbour_index = to_index(neighbour, size);
            let new_dist = dist + graph[neighbour.1][neighbour.0];
            if distance[neighbour_index].map_or(true, |old_dist| new_dist < old_dist) {
                heap.push(Reverse((new_dist, neighbour)));
                distance[neighbour_index] = Some(new_dist);
                predecessors[neighbour_index] = Some(pos);
            }
        }
    }
    distance[to_index(end, size)].unwrap_or(u32::MAX)
}

fn extend_graph(graph: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let size = graph.len();
    let mut extended = vec![vec![0; size * 5]; size * 5];
    for roff in 0..5 {
        for (r, row) in graph.iter().enumerate() {
            for coff in 0..5 {
                for (c, &val) in row.iter().enumerate() {
                    let mut new_val = val;
                    for _ in 0..(coff + roff) {
                        new_val = if new_val == 9 { 1 } else { new_val + 1 };
                    }
                    extended[roff * size + r][coff * size + c] = new_val;
                }
            }
        }
    }
    extended
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let lines = utils::read_file(&args[1]);

    let rows: Vec<Vec<u32>> = lines
        .iter()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let total_risk = shortest_path(&rows, (0, 0), (rows.len() - 1, rows.len() - 1));
    println!("Risk: {}", total_risk);

    let extended_graph = extend_graph(rows);
    let total_risk_extended = shortest_path(
        &extended_graph,
        (0, 0),
        (extended_graph.len() - 1, extended_graph.len() - 1),
    );
    println!("Risk: {}", total_risk_extended);
}
