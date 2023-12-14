use aoc::*;
use std::io;

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

fn find_mirror<T>(grid: &Grid<T>, errors: i32) -> Option<usize>
where
    T: Eq,
{
    (1..grid.rows)
        .filter_map(|i| {
            let mut err = errors;
            (grid.as_rows().take(i)
                .rev()
                .zip(grid.as_rows().skip(i))
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
        .map_v(|pat| pat.lines().map(|n| n.chars_v()).to_grid());
    let rots = pats.map_v(Grid::transpose_clone);

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
