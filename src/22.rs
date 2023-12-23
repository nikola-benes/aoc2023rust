use aoc::*;
use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Brick {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    min_z: i32,
    max_z: i32,
}

fn fall(bricks: &mut [Brick]) -> i32 {
    let mut top = HashMap::<(i32, i32), i32>::new();
    let mut fallen = 0;

    for b in bricks {
        let mut fall = b.min_z;
        for x in b.min_x..=b.max_x {
            for y in b.min_y..=b.max_y {
                fall = fall.min(b.min_z - top.get(&(y, x)).unwrap_or(&0) - 1);
            }
        }

        if fall > 0 {
            fallen += 1;
        }

        b.min_z -= fall;
        b.max_z -= fall;

        for x in b.min_x..=b.max_x {
            for y in b.min_y..=b.max_y {
                top.insert((y, x), b.max_z);
            }
        }
    }

    fallen
}

fn main() {
    let mut bricks = lines().map_v(|line| {
        let mut it = line.split(&[',', '~']).map(|n| n.parse_());
        let (min_x, min_y, min_z) = it.to_triple();
        let (max_x, max_y, max_z) = it.to_triple();
        Brick {
            min_x,
            max_x,
            min_y,
            max_y,
            min_z,
            max_z,
        }
    });

    bricks.sort_by_key(|b| b.min_z);

    fall(&mut bricks);

    let falls = (0..bricks.len())
        .map_v(|i| fall(&mut [&bricks[0..i], &bricks[i + 1..]].concat()));

    println!("{}", falls.filter(|&&f| f == 0).count());
    println!("{}", falls.into_iter().sum_());
}
