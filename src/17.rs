use aoc::*;
use std::collections::{BinaryHeap, HashSet};

// N E S W
const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn solve(map: &Grid<i32>, min_steps: i32, max_steps: i32) -> i32 {
    // priority (= -distance), (y, x, dir, steps)
    let start = (0, (0, 0, 1, 0));
    let goal = (map.rows as i32 - 1, map.cols as i32 - 1);

    let mut pq = BinaryHeap::new();
    let mut seen = HashSet::new();
    pq.push(start);

    while let Some((p, state)) = pq.pop() {
        if seen.contains(&state) {
            continue;
        }

        let (y, x, dir, steps) = state;
        if (y, x) == goal && steps >= min_steps {
            return -p;
        }

        seen.insert(state);

        for ndir in 0..4 {
            if ndir == (dir + 2) % 4
                || (dir == ndir && steps == max_steps)
                || (dir != ndir && 0 < steps && steps < min_steps)
            {
                continue;
            }

            let nsteps = if dir == ndir { steps + 1 } else { 1 };
            let (dy, dx) = DIRS[ndir as usize];
            let (ny, nx) = (y + dy, x + dx);
            let nstate = (ny, nx, ndir, nsteps);

            if map.valid_coords(ny, nx) && !seen.contains(&nstate) {
                pq.push((p - map[(ny, nx)], nstate));
            }
        }
    }

    panic!();
}

fn main() {
    let map = lines()
        .map(|line| line.chars().map_v(|c| c as i32 - '0' as i32))
        .to_grid();

    println!("{}", solve(&map, 0, 3));
    println!("{}", solve(&map, 4, 10));
}
