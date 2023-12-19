use aoc::*;

const DIRS: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
const N: u8 = 0b0001;
const E: u8 = 0b0010;
const S: u8 = 0b0100;
const W: u8 = 0b1000;

// Pick's theorem:
//   A = i + b / 2 - 1
// where
//   A = area of polygon with integer vertex coordinates
//   i = interior points with integer coordinates
//   b = boundary points with integer coordinates
//
// thus: i = A - b / 2 + 1
//
// we need to compute b + i = A + b / 2 + 1
// to compute A, we sum oriented triangles (0, 0) -- (x, y) -- (next_x, next_y)
// each such area is (x * next_y - next_x * y) / 2

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

fn solve(plan: &Vec<(u8, i64, String)>) -> i64 {
    let mut last = (0, 0);
    let twice_area = plan
        .map(|&(dir, len, _)| {
            let (dy, dx) = DIRS[dir.trailing_zeros() as usize];
            let (ly, lx) = last;
            let (ny, nx) = (ly + dy * len, lx + dx * len);
            last = (ny, nx);
            lx * ny - nx * ly
        })
        .sum_();
    let boundary = plan.map(|&(_, len, _)| len).sum_();
    twice_area / 2 + boundary / 2 + 1
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
