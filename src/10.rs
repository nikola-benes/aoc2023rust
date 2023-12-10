use aoc::*;
use std::collections::{hash_map::Entry::Vacant, HashMap, VecDeque};

// (dy, dx)
const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
const N: u8 = 0b0001;
const E: u8 = 0b0010;
const S: u8 = 0b0100;
const W: u8 = 0b1000;
const START: u8 = 0b1111;

const PIPES: [(char, u8); 8] = [
    ('|', N | S),
    ('-', E | W),
    ('L', N | E),
    ('J', N | W),
    ('7', S | W),
    ('F', S | E),
    ('.', 0),
    ('S', START),
];

fn neighbours(
    (y, x): (i32, i32),
    grid: &Vec<Vec<u8>>,
) -> impl Iterator<Item = (i32, i32)> {
    let pipe = grid[y as usize][x as usize];
    DIRS.iter().enumerate().filter_map(move |(d, &(dy, dx))| {
        if pipe & (1 << d) != 0 {
            Some((y + dy, x + dx))
        } else {
            None
        }
    })
}

fn main() {
    let pipes = HashMap::from(PIPES);
    let mut grid = lines().map_v(|line| line.chars().map_v(|c| pipes[&c]));
    let (sy_u, sx_u) = grid
        .iter()
        .enumerate()
        .filter_map(|(y, row)| {
            row.iter().position(|&p| p == START).map(|x| (y, x))
        })
        .next_();
    let (sy, sx) = (sy_u as i32, sx_u as i32);

    // fix S
    for (d, &(dy, dx)) in DIRS.iter().enumerate() {
        if sy + dy < 0
            || sy + dy >= grid.len() as i32
            || sx + dx < 0
            || sx + dx >= grid[0].len() as i32
            || grid[(sy + dy) as usize][(sx + dx) as usize]
                & (1 << ((d + 2) % 4))
                == 0
        {
            grid[sy_u][sx_u] &= !(1 << d);
        }
    }

    let mut dist = HashMap::from([((sy, sx), 0)]);
    let mut q = VecDeque::from([(sy, sx)]);
    let mut part1 = 0;

    while let Some(p) = q.pop_front() {
        let d = dist[&p];
        part1 = d;
        for n in neighbours(p, &grid) {
            if let Vacant(entry) = dist.entry(n) {
                entry.insert(d + 1);
                q.push_back(n);
            }
        }
    }
    println!("{}", part1);

    let mut part2 = 0;
    let mut inside = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, &tile) in row.iter().enumerate() {
            if dist.contains_key(&(y as i32, x as i32)) {
                inside ^= tile & (N | S);
            } else if inside != 0 {
                part2 += 1;
            }
        }
    }
    println!("{}", part2);
}
