use aoc::*;
use std::collections::HashSet;

fn main() {
    let cards = lines().map_v(|line| {
        let (w, h) = line
            .split(": ")
            .nth_(1)
            .split("|")
            .map(|nums| {
                nums.split_whitespace()
                    .map(|n| n.parse_::<i32>())
                    .collect::<HashSet<_>>()
            })
            .to_pair();
        // Sprinkle some & on the code until it compiles…
        w.intersection(&h).count()
    });

    let mut part1 = 0;
    for c in &cards {
        // Sprinkle some ‹as› and * on the code until it compiles…
        part1 += (2 as i32).pow(*c as u32) / 2;
    }

    let mut copies = vec![1; cards.len()];
    for (i, c) in cards.iter().enumerate() {
        let mult = copies[i];
        for j in i + 1..(i + 1 + c).min(copies.len()) {
            copies[j] += mult
        }
    }

    println!("{}", part1);
    println!("{}", copies.into_iter().sum_());
}
