use aoc::*;
use regex::Regex;
use std::collections::HashMap;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b)
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn main() {
    let word = Regex::new("[A-Z]+").unwrap();
    let mut it = lines();
    let path = it.next_();
    it.next_();

    let net = it
        .map(|line| {
            let (dst, left, right) = word
                .find_iter(&line)
                // What an empowering ‹.as_str().to_string()›!
                .map(|m| m.as_str().to_string())
                .to_triple();
            (dst, (left, right))
        })
        .collect::<HashMap<_, _>>();

    // Removing the type of ‹node› makes ‹get_path("AAA")› fail.
    let next = |node: &str, dir| {
        if dir == 'L' {
            &net[node].0
        } else {
            &net[node].1
        }
    };

    // If I try to manually inline ‹next› inside ‹get_path›, I get various
    // funny lifetime errors which I am unable to deal with… :empowered:
    let get_path = |mut node| {
        path.chars().cycle().map(move |dir| {
            node = next(node, dir);
            node
        })
    };

    println!(
        "{}",
        1 + get_path("AAA").take_while(|&node| node != "ZZZ").count()
    );

    // Experiments with the input show that all ghost paths are cycles
    // (although generally they could have been lassoes).
    println!(
        "{}",
        net.keys()
            .filter(|k| k.ends_with('A'))
            .map(|node| {
                1 + get_path(node)
                    .take_while(|&node| !node.ends_with('Z'))
                    .count() as u64
            })
            .reduce_(lcm)
    );
}
