use aoc::*;
use std::collections::HashMap;

fn fall(g: &mut Grid<char>, (dy, dx): (i32, i32)) {
    let sy = if dy == 1 { g.rows as i32 - 1 } else { 0 };
    let sx = if dx == 1 { g.cols as i32 - 1 } else { 0 };
    let mut y = sy;
    let mut x = sx;

    let mut ty = y;
    let mut tx = x;

    while g.valid_coords(y, x) {
        if g[(y, x)] == '#' {
            ty = y - dy;
            tx = x - dx;
        } else if g[(y, x)] == 'O' {
            g.swap((y, x), (ty, tx));
            ty -= dy;
            tx -= dx;
        }

        y -= dy;
        x -= dx;

        if dx == 0 && !g.valid_row(y) {
            x += 1;
            tx += 1;
            y = sy;
            ty = sy;
        } else if dy == 0 && !g.valid_col(x) {
            y += 1;
            ty += 1;
            x = sx;
            tx = sx;
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
