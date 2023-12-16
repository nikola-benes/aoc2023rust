use aoc::*;
use std::collections::VecDeque;

const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
const N: u8 = 0b0001;
const E: u8 = 0b0010;
const S: u8 = 0b0100;
const W: u8 = 0b1000;

fn bit_indices(mut num: u8) -> impl Iterator<Item = usize> {
    std::iter::from_fn(move || {
        let i = num.trailing_zeros() as usize;
        num &= num.wrapping_sub(1);
        (i != 8).then_some(i)
    })
}

fn apply_tile(beam: u8, tile: char) -> u8 {
    let split = |dirs| if (beam & dirs) != 0 { beam } else { dirs };
    let mirror = |mut dirs| {
        if (beam & dirs) == 0 {
            dirs ^= 0b1111;
        }
        beam ^ dirs
    };
    match tile {
        '.' => beam,
        '-' => split(E | W),
        '|' => split(N | S),
        '/' => mirror(N | E),
        '\\' => mirror(N | W),
        _ => panic!(),
    }
}

fn solve_from(cave: &Grid<char>, y: usize, x: usize, dir: u8) -> usize {
    let (y, x) = (y as i32, x as i32);
    let mut beam = Grid::new(cave.rows, cave.cols, 0u8);
    let mut q = VecDeque::new();

    q.push_back((y, x, dir));
    beam[(y, x)] |= dir;

    while let Some((y, x, dir)) = q.pop_front() {
        for i in bit_indices(apply_tile(dir, cave[(y, x)])) {
            let ndir = 1 << i;
            let (dy, dx) = DIRS[i];
            let (ny, nx) = (y + dy, x + dx);
            if 0 <= ny
                && ny < cave.rows as i32
                && 0 <= nx
                && nx < cave.cols as i32
                && beam[(ny, nx)] & ndir == 0
            {
                q.push_back((ny, nx, ndir));
                beam[(ny, nx)] |= ndir;
            }
        }
    }

    beam.as_tiles().filter(|&&b| b != 0).count()
}

fn main() {
    let cave = lines().map(|line| line.chars_v()).to_grid();
    println!("{}", solve_from(&cave, 0, 0, E));

    let mut part2 = 0;
    for y in 0..cave.rows {
        part2 = part2.max(solve_from(&cave, y, 0, E));
        part2 = part2.max(solve_from(&cave, y, cave.cols - 1, W));
    }
    for x in 0..cave.cols {
        part2 = part2.max(solve_from(&cave, 0, x, S));
        part2 = part2.max(solve_from(&cave, cave.rows - 1, x, N));
    }
    println!("{}", part2);
}
