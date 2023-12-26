use aoc::*;
use std::collections::{HashMap, HashSet, VecDeque};

const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn to_dir(d: char) -> usize {
    match d {
        '^' => 0,
        '>' => 1,
        'v' => 2,
        '<' => 3,
        _ => panic!(),
    }
}

type Graph = HashMap<(i32, i32), Vec<((i32, i32), i32)>>;

fn longest_dag_path(dag: &Graph, goal: (i32, i32)) -> i32 {
    let mut ins = HashMap::new();
    for (_, to) in dag {
        for (dst, _) in to {
            *ins.entry(dst).or_insert(0) += 1;
        }
    }
    let mut q = VecDeque::new();
    let mut dist = HashMap::new();
    for (src, _) in dag {
        if !ins.contains_key(src) {
            q.push_back(src);
            dist.insert(src, 0);
        }
    }
    while let Some(src) = q.pop_front() {
        let Some(to) = dag.get(src) else { continue };
        let current = dist[src];
        for (dst, d) in to {
            // relax src -d-> dst
            let new = current + d;
            dist.entry(dst)
                .and_modify(|old| *old = new.max(*old))
                .or_insert(new);

            let in_dst = ins.get_mut(dst).unwrap();
            *in_dst -= 1;
            if *in_dst == 0 {
                q.push_back(dst);
            }
        }
    }
    dist[&goal]
}

// Note: Memoisation actually makes things much worse here…
fn longest_path_rec(
    graph: &Graph,
    init: (i32, i32),
    goal: (i32, i32),
    seen: &mut HashSet<(i32, i32)>,
) -> i32 {
    if init == goal {
        return 0;
    }

    let mut result = -1;
    for &(dst, d) in &graph[&init] {
        if !seen.insert(dst) {
            continue;
        }
        let next = longest_path_rec(graph, dst, goal, seen);
        if next != -1 {
            result = result.max(d + next);
        }
        seen.remove(&dst);
    }

    result
}

fn longest_path(graph: &Graph, init: (i32, i32), goal: (i32, i32)) -> i32 {
    longest_path_rec(graph, init, goal, &mut HashSet::from([init]))
}

fn main() {
    let map = lines().map(|line| line.chars_v()).to_grid();
    let ty = map.rows as i32 - 1;
    let sx = map
        .as_rows()
        .next_()
        .enumerate()
        .filter_map(|(i, &c)| (c == '.').then_some(i))
        .next_() as i32;

    let mut graph = Graph::new();
    let mut seen = HashSet::new();

    let mut q = VecDeque::new();
    let init = (0, sx);
    let mut goal = (-1, -1);

    q.push_back(((1, sx), init, init, 1));

    while let Some(((y, x), prev, mut start, d)) = q.pop_front() {
        for (i, (dy, dx)) in DIRS.enumerate() {
            let (ny, nx) = (y + dy, x + dx);
            let ntile = map[(ny, nx)];
            if (ny, nx) == prev
                || ntile == '#'
                || (ntile != '.' && i != to_dir(ntile))
            {
                continue;
            }

            let mut nd = d + 1;
            if (map[(y, x)] != '.' && d > 1) || ny == ty {
                graph
                    .entry(start)
                    .or_insert(Vec::new())
                    .push(((ny, nx), nd));
                nd = 0;
                start = (ny, nx);
            }

            if ny == ty {
                goal = (ny, nx);
            } else if seen.insert((ny, nx)) {
                q.push_back(((ny, nx), (y, x), start, nd));
            }
        }
    }

    println!("{}", longest_dag_path(&graph, goal));

    let mut new_graph = Graph::new();
    for (src, to) in graph {
        for (dst, d) in to {
            for (a, b) in [(src, dst), (dst, src)] {
                new_graph.entry(a).or_insert(Vec::new()).push((b, d));
            }
        }
    }

    println!("{}", longest_path(&new_graph, init, goal));
}
