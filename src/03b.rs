mod aoc;
use aoc::*;
use std::collections::HashSet;

const DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn main() {
    let schema = lines().map_v(|line| line.trim_end().chars_v());
    let (rows, cols) = (schema.len() as i32, schema[0].len() as i32);
    let mut nums = Vec::new();
    let mut num_at = vec![vec![-1; cols as usize]; rows as usize];

    let get = |y: i32, x: i32| {
        if y < 0 || y >= rows || x < 0 || x >= cols {
            '.'
        } else {
            schema[y as usize][x as usize]
        }
    };

    for y in 0..rows {
        let mut x = 0;
        while x < cols {
            while x < cols && !get(y, x).is_ascii_digit() {
                x += 1
            }
            if x == cols {
                break;
            }
            let start = x;
            let mut n = String::new();
            while x < cols && get(y, x).is_ascii_digit() {
                n.push(get(y, x));
                x += 1;
            }
            let num = n.parse_::<i32>();
            let ix = nums.len() as i32;
            nums.push(num);
            for nx in start..x {
                num_at[y as usize][nx as usize] = ix;
            }
        }
    }

    let num_get = |y: i32, x: i32| {
        if y < 0 || y >= rows || x < 0 || x >= cols {
            None
        } else {
            let num = num_at[y as usize][x as usize];
            if num == -1 {
                None
            } else {
                Some(num)
            }
        }
    };

    let mut part2 = 0;

    for y in 0..rows {
        for x in 0..cols {
            let c = get(y, x);
            if c == '*' {
                let ns = DIRS
                    .iter()
                    .filter_map(|(dy, dx)| num_get(y + dy, x + dx))
                    .collect::<HashSet<_>>();
                if ns.len() == 2 {
                    part2 += ns
                        .into_iter()
                        .map(|ix| nums[ix as usize])
                        .product::<i32>();
                }
            }
        }
    }

    println!("{}", part2);
}
