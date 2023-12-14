use aoc::*;
use std::collections::HashMap;

fn fall(g: &mut Grid<char>, (dy, dx): (i32, i32)) {
    let mut y = if dy == 1 { g.rows as i32 - 2 } else { -dy };
    let mut x = if dx == 1 { g.cols as i32 - 2 } else { -dx };

    let rows = g.rows as i32;
    let cols = g.cols as i32;

    let valid = |y, x| 0 <= y && y < rows && 0 <= x && x < cols;

    while valid(y, x) {
        if g[(y, x)] == 'O' {
            g[(y, x)] = '.';
            while valid(y + dy, x + dx) && g[(y + dy, x + dx)] == '.' {
                y += dy;
                x += dx;
            }
            g[(y, x)] = 'O';
        }

        if dx == 0 {
            x += 1;
            if x as usize == g.cols {
                x = 0;
                y -= dy;
            }
        } else {
            y += 1;
            if y as usize == g.rows {
                y = 0;
                x -= dx;
            }
        }
    }
}

fn load(g: &Grid<char>) -> usize {
    g.as_rows()
        .rev()
        .zip(1..)
        .map(|(row, m)| row.filter(|&&c| c == 'O').count() * m)
        .sum_()
}

fn main() {
    let start = lines().map(|line| line.chars_v()).to_grid();

    let mut g = start.clone();
    fall(&mut g, (-1, 0));
    println!("{}", load(&g));

    g = start;
    let mut seen = HashMap::new();
    let mut stored = vec![g.clone()];
    let goal: usize = 1000000000;

    for round in 1.. {
        for d in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            fall(&mut g, d);
        }
        if let Some(&start) = seen.get(&g) {
            println!(
                "{}",
                load(&stored[start + (goal - start) % (round - start)])
            );
            break;
        }
        seen.insert(g.clone(), round);
        stored.push(g.clone());
    }
}
