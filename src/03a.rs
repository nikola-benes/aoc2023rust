mod aoc;
use aoc::*;

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
    let mut touch = vec![vec![false; cols as usize]; rows as usize];

    let mut tset = |y: i32, x: i32, b: bool| {
        if 0 <= y && y < rows && 0 <= x && x < cols {
            touch[y as usize][x as usize] = b;
        }
    };

    let get = |y: i32, x: i32| {
        if y < 0 || y >= rows || x < 0 || x >= cols {
            '.'
        } else {
            schema[y as usize][x as usize]
        }
    };

    for y in 0..rows {
        for x in 0..cols {
            let c = get(y, x);
            if c != '.' && !c.is_ascii_digit() {
                for (dy, dx) in DIRS {
                    tset(y + dy, x + dx, true);
                }
            }
        }
    }

    let mut part1 = 0;
    for y in 0..rows {
        let mut x = 0;
        while x < cols {
            while x < cols && !get(y, x).is_ascii_digit() {
                x += 1
            }
            if x == cols {
                break;
            }
            let mut n = String::new();
            let mut touched = false;
            while x < cols && get(y, x).is_ascii_digit() {
                n.push(get(y, x));
                if touch[y as usize][x as usize] {
                    touched = true;
                }
                x += 1;
            }
            if touched {
                part1 += n.parse_::<i32>();
            }
        }
    }

    println!("{}", part1);
}
