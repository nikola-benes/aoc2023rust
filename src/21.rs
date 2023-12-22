use aoc::*;
use std::collections::{hash_map::Entry::Vacant, HashMap, VecDeque};

const DIRS: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn main() {
    let map = lines().map(|line| line.chars_v()).to_grid();
    let start = map
        .enumerate_tiles()
        .filter_map(|(y, x, &tile)| {
            (tile == 'S').then_some((y as i64, x as i64))
        })
        .next_();

    let mut dist = HashMap::new();
    let mut q = VecDeque::new();
    dist.insert(start, 0);
    q.push_back((start, 0));

    let steps = 26501365;
    let cycle = 2 * map.rows; // found experimentally
    let mut next = steps % cycle;
    let mut counts = Vec::new();
    let mut p1_done = false;

    while let Some(((y, x), d)) = q.pop_front() {
        if d == 64 && !p1_done {
            println!("{}", dist.filter(|&(_, d)| d % 2 == 0).count());
            p1_done = true;
        }
        if d == next {
            counts.push(dist.filter(|&(_, d)| d % 2 == steps % 2).count());
            if counts.len() == 3 {
                break;
            }
            next += cycle;
        }
        for (dy, dx) in DIRS {
            let (ny, nx) = (y + dy, x + dx);
            let pos = (
                ny.rem_euclid(map.rows as i64),
                nx.rem_euclid(map.cols as i64),
            );
            if map[pos] != '#' {
                if let Vacant(entry) = dist.entry((ny, nx)) {
                    entry.insert(d + 1);
                    q.push_back(((ny, nx), d + 1));
                }
            }
        }
    }

    let mut last = counts[2];
    let mut diff = counts[2] - counts[1];
    let ddiff = diff - (counts[1] - counts[0]);

    for _ in 3..=steps / cycle {
        diff = diff + ddiff;
        last = last + diff;
    }
    println!("{}", last);
}
