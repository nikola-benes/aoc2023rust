use aoc::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;

fn draw(graph: &HashMap<String, Vec<String>>) {
    println!("graph {{");
    for (v, vs) in graph {
        for w in vs {
            println!("{} -- {}", v, w);
        }
    }
    println!("}}");
}

fn main() {
    let input: HashMap<_, _> = lines()
        .map(|line| {
            let (v, vs) = line.split(": ").to_pair();
            (v.to_string(), vs.split(' ').map_v(|w| w.to_string()))
        })
        .collect();

    if env::args().nth(1).is_some_and(|x| x == "-d") {
        return draw(&input);
    }

    let mut graph = input.clone();
    for (v, vs) in input {
        for w in vs {
            graph.entry(w).or_insert(Vec::new()).push(v.clone());
        }
    }

    // edges found by manually examining the output of ‹neato›
    let edges = if graph.len() == 15 {
        [("pzl", "hfx"), ("cmg", "bvb"), ("nvd", "jqt")]
    } else {
        [("pzc", "vps"), ("cvx", "dph"), ("sgc", "xvk")]
    };

    for (v, w) in edges {
        for (v, w) in [(v, w), (w, v)] {
            let vs = graph.get_mut(v).unwrap();
            let i = vs.position(|x| x == w).unwrap();
            vs.remove(i);
        }
    }

    let start = graph.keys().next_();
    let mut seen = HashSet::from([start]);
    let mut q = VecDeque::from([start]);

    while let Some(v) = q.pop_front() {
        for w in &graph[v] {
            if seen.insert(&w) {
                q.push_back(&w);
            }
        }
    }

    println!("{}", seen.len() * (graph.len() - seen.len()));
}
