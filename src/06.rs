mod aoc;
use aoc::*;
use std::iter::zip;

fn solve(t: i64, d: i64) -> i64 {
    // (t - x) * x > d
    // -x² + tx - d > 0
    // x₁ = (t - sqrt(t² - 4d)) / 2
    // let z be the first integer > x₁
    // then the valid options are [z, t - z], i.e. t - 2z + 1 options in total
    let ft = t as f64;
    let fd = d as f64;
    let low = (ft - (ft * ft - 4.0 * fd).sqrt()) / 2.0;
    let first = low.floor() as i64 + 1;
    t - 2 * first + 1
}

fn main() {
    let lines = lines().map_v(|line| line.split(": ").nth_(1).to_string());
    let (ts, ds) = lines
        .iter()
        .map(|line| line.split_whitespace().map(|n| n.parse_()))
        .to_pair();

    println!("{}", zip(ts, ds).map(|(t, d)| solve(t, d)).product_());

    let (t, d) = lines
        .into_iter()
        .map(|line| {
            line.chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
                .parse_()
        })
        .to_pair();

    println!("{}", solve(t, d));
}
