use aoc::*;
use std::collections::BTreeMap;

const DIRS: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
const N: u8 = 0b0001;
const E: u8 = 0b0010;
const S: u8 = 0b0100;
const W: u8 = 0b1000;

fn normalise_dir(d: char) -> u8 {
    match d {
        'U' => N,
        'R' => E,
        'D' => S,
        'L' => W,
        '0' => E,
        '1' => S,
        '2' => W,
        '3' => N,
        _ => panic!(),
    }
}

fn opp_dir(d: u8) -> u8 {
    if d & (N | S) != 0 {
        d ^ (N | S)
    } else {
        d ^ (E | W)
    }
}

fn solve(plan: &Vec<(u8, i64, String)>) -> i64 {
    let mut dig = BTreeMap::new();
    let (mut y, mut x) = (0, 0);
    for &(dir, len, _) in plan {
        let (dy, dx) = DIRS[dir.trailing_zeros() as usize];
        for _ in 0..len {
            *dig.entry((y, x)).or_insert(0) |= dir;
            y += dy;
            x += dx;
            *dig.entry((y, x)).or_insert(0) |= opp_dir(dir);
        }
    }

    let mut total = dig.len() as i64;
    let mut inside = 0;
    let mut last = 0;
    for (&(_, x), &tile) in dig.iter() {
        if inside != 0 {
            total += x - last - 1
        }
        inside ^= tile & (N | S);
        last = x;
    }

    total
}

fn main() {
    let mut plan = lines().map_v(|line| {
        let (d, len, c) = line.split_whitespace().to_triple();
        (
            normalise_dir(d.chars().next_()),
            len.parse_::<i64>(),
            c.to_string(),
        )
    });

    println!("{}", solve(&plan));

    for (d, len, c) in plan.iter_mut() {
        let mut it = c.chars().skip(2);
        let mut digits = String::new();
        for _ in 0..5 {
            digits.push(it.next_());
        }
        *len = i64::from_str_radix(&digits, 16).unwrap();
        *d = normalise_dir(it.next_());
    }

    println!("{}", solve(&plan));
}
