mod aoc;
use aoc::*;
use std::collections::VecDeque;
use std::io;

fn main() {
    // So we have lines(), but we don't have an iterator over parts of files
    // with a different delimiter? How empowering.
    let input_s = io::read_to_string(io::stdin()).unwrap();
    let mut input = input_s.split("\n\n");
    let initial = input
        .next_()
        .split(": ")
        .nth_(1)
        .split_whitespace()
        .map_v(|n| n.parse_::<i64>());

    let maps = input.map_v(|m| {
        m.lines().skip(1).map_v(|line| {
            line.split_whitespace()
                .map(|n| n.parse_::<i64>())
                .to_triple()
        })
    });

    // Part 1
    let mut seeds = initial.clone();
    for changes in maps.iter() {
        let mut new_seeds = seeds.clone();
        for (dst, src, len) in changes.iter() {
            for (i, seed) in seeds.iter().enumerate() {
                // Autoderef works in arithmetic operators,
                // but not in comparison operators… curiouser and curiouser.
                if src <= seed && *seed < src + len {
                    new_seeds[i] = seed + dst - src;
                }
            }
        }
        seeds = new_seeds;
    }

    println!("{}", seeds.into_iter().reduce_(i64::min));

    // Part 2
    let mut seeds = VecDeque::new();
    let mut it = initial.into_iter();

    while let (Some(start), Some(len)) = (it.next(), it.next()) {
        seeds.push_back((start, start + len));
    }

    for changes in maps.into_iter() {
        let mut new_seeds = VecDeque::new();
        while let Some((from, mut to)) = seeds.pop_front() {
            for (dst, src, len) in changes.iter() {
                // Autoderefences, what fun!
                let (m_from, m_to) = (from.max(*src), to.min(src + len));
                if m_from >= m_to {
                    continue;
                }
                new_seeds.push_front((dst + m_from - src, dst + m_to - src));
                if m_to < to {
                    seeds.push_back((m_to, to));
                }
                to = to.min(m_from);
                if from == to {
                    break;
                }
            }
            if from < to {
                new_seeds.push_back((from, to));
            }
        }
        seeds = new_seeds;
    }

    println!("{}", seeds.into_iter().map(|(s, _)| s).reduce_(i64::min));
}
