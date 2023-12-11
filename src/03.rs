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

fn get2d<T>(v: &Vec<Vec<T>>, y: i32, x: i32, def: T) -> T
where
    T: Copy,
{
    if y < 0 || y >= v.len() as i32 || x < 0 || x >= v[0].len() as i32 {
        def
    } else {
        v[y as usize][x as usize]
    }
}

fn main() {
    let schema = lines().map_v(|line| line.trim_end().chars_v());
    let (rows, cols) = (schema.len() as i32, schema[0].len() as i32);
    let mut nums = Vec::new();
    let mut num_at = vec![vec![None; cols as usize]; rows as usize];

    let get = |y: i32, x: i32| get2d(&schema, y, x, '.');

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
                num_at[y as usize][nx as usize] = Some(ix);
            }
        }
    }

    let num_get = |y: i32, x: i32| get2d(&num_at, y, x, None);

    let mut part2 = 0;
    let mut touched = HashSet::new();

    for y in 0..rows {
        for x in 0..cols {
            let c = get(y, x);
            if c != '.' && !c.is_ascii_digit() {
                let ns = DIRS
                    .iter()
                    .filter_map(|(dy, dx)| num_get(y + dy, x + dx))
                    .collect::<HashSet<_>>();
                if c == '*' && ns.len() == 2 {
                    part2 += ns
                        // Sprinkle some * on the code until it compiles…
                        .map(|ix| nums[*ix as usize])
                        .product_();
                }
                touched.extend(ns.into_iter());
            }
        }
    }

    println!("{}", touched.into_iter().map(|n| nums[n as usize]).sum_());
    println!("{}", part2);
}
