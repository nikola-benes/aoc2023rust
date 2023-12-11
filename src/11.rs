use aoc::*;

fn main() {
    let image = lines().map_v(|line| line.chars_v());
    let empty_rows = image
        .enumerate()
        .filter_map_v(|(i, line)| line.all(|&c| c == '.').then_some(i as i64));
    let empty_cols = (0..image[0].len()).filter_map_v(|i| {
        image.all(|line| line[i] == '.').then_some(i as i64)
    });

    let orig_stars = image
        .enumerate()
        .map(|(y, line)| {
            line.enumerate().filter_map(move |(x, &c)| {
                (c == '#').then_some((y as i64, x as i64))
            })
        })
        .flatten_v();

    for d in [1, 999999] {
        let mut stars = orig_stars.clone();
        for (y, x) in &mut stars {
            *y += empty_rows.partition_point(|&r| r < *y) as i64 * d;
            *x += empty_cols.partition_point(|&r| r < *x) as i64 * d;
        }

        println!(
            "{}",
            stars
                .enumerate()
                .map(|(i, (ay, ax))| stars[i + 1..]
                    .map(move |(by, bx)| (bx - ax).abs() + (by - ay).abs()))
                .flatten()
                .sum_()
        );
    }
}
