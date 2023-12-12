use aoc::*;
use std::collections::HashMap;

fn solve_memo(
    memo: &mut HashMap<(usize, usize), i64>,
    pixels: &[char],
    groups: &[usize],
    mut p: usize,
    g: usize,
) -> i64 {
    // Cannot use ‹entry› here as that would need two mutable borrows
    // of ‹memo› at once.
    if let Some(&v) = memo.get(&(p, g)) {
        return v;
    }
    let result = 'r: {
        while p < pixels.len() && pixels[p] == '.' {
            p += 1
        }
        if p >= pixels.len() {
            break 'r (g >= groups.len()) as i64;
        }
        if g >= groups.len() {
            break 'r pixels[p..].all(|&x| x != '#') as i64;
        }

        let size = groups[g];
        let mut result = 0;

        // '#' or '?', try building one group
        if p + size <= pixels.len()
            && pixels[p..p + size].all(|&x| x != '.')
            && (p + size == pixels.len() || pixels[p + size] != '#')
        {
            result += solve_memo(memo, pixels, groups, p + size + 1, g + 1);
        }

        // try ignoring current '?'
        if pixels[p] == '?' {
            result += solve_memo(memo, pixels, groups, p + 1, g);
        }

        result
    };
    memo.insert((p, g), result);
    result
}

fn solve(pixels: &str, groups: &str) -> i64 {
    solve_memo(
        &mut HashMap::new(),
        &pixels.chars_v(),
        &groups.split(',').map_v(|n| n.parse_()),
        0,
        0,
    )
}

fn unfold(orig: &str, sep: char, n: usize) -> String {
    // ‹std› is of not much help here…
    let mut r = orig.to_string();
    for _ in 1..n {
        r.push(sep);
        r.push_str(orig);
    }
    r
}

fn main() {
    let input = lines()
        .map_v(|line| line.split_whitespace().map(str::to_string).to_pair());

    println!("{}", input.map(|(p, g)| solve(p, g)).sum_());
    println!(
        "{}",
        input
            .map(|(p, g)| solve(&unfold(p, '?', 5), &unfold(g, ',', 5)))
            .sum_()
    );
}
