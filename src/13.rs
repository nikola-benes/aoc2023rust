use aoc::*;
use std::io;

fn transpose<T>(grid: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    let cols = grid[0].len();
    let mut its = grid.map_v(|row| row.iter());
    (0..cols).map_v(|_| its.iter_mut().map_v(|row| row.next_().clone()))
}

fn almost_eq<T>(xs: &[T], ys: &[T], err: &mut i32) -> bool
where
    T: Eq,
{
    xs.zip(ys).all(|(x, y)| {
        x == y
            || (*err > 0 && {
                *err -= 1;
                true
            })
    })
}

fn find_mirror<T>(grid: &[Vec<T>], errors: i32) -> Option<usize>
where
    T: Eq,
{
    (1..grid.len())
        .filter_map(|i| {
            let mut err = errors;
            (grid[0..i]
                .rev()
                .zip(&grid[i..])
                .all(|(x, y)| almost_eq(x, y, &mut err))
                && err == 0)
                .then_some(i)
        })
        .next()
}

fn main() {
    let pats = io::read_to_string(io::stdin())
        .unwrap()
        .split("\n\n")
        .map_v(|pat| pat.lines().map_v(|n| n.chars_v()));
    let rots = pats.map_v(transpose);

    for err in [0, 1] {
        println!(
            "{}",
            [&pats, &rots]
                .zip([100, 1])
                .map(|(&ps, m)| ps
                    .filter_map(move |p| find_mirror(p, err).map(|i| i * m)))
                .flatten()
                .sum_()
        );
    }
}
