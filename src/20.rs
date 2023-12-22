use aoc::*;
use regex::Regex;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
enum Mod {
    FlipFlop(bool),
    Nand(u16),
    Broadcast,
}

impl Mod {
    fn new(typ: &str) -> Mod {
        match typ {
            "" => Mod::Broadcast,
            "%" => Mod::FlipFlop(false),
            "&" => Mod::Nand(0),
            _ => panic!(),
        }
    }
}

fn push_button(
    inputs: &HashMap<String, Vec<String>>,
    state: &mut HashMap<String, (Mod, Vec<String>)>,
    to_watch: &str,
) -> (i64, i64, Option<usize>) {
    let mut q = VecDeque::new();
    let mut count = [0, 0];
    let mut high_from = None;

    // We need to use ‹String›s here to avoid borrowing from ‹state› twice.
    q.push_back(("button".to_string(), false, "broadcaster".to_string()));

    while let Some((source, pulse, target)) = q.pop_front() {
        count[pulse as usize] += 1;

        let Some((m, targets)) = state.get_mut(&target) else {
            continue;
        };

        let new_pulse = match m {
            Mod::Broadcast => pulse,
            Mod::FlipFlop(ref mut state) => {
                if pulse {
                    continue;
                }
                *state = !*state;
                *state
            }
            Mod::Nand(ref mut state) => {
                let ins = &inputs[&target];
                let ix = ins.position(|x| *x == source).unwrap();
                if pulse {
                    if target == to_watch {
                        high_from = Some(ix);
                    }
                    *state |= 1 << ix;
                } else {
                    *state &= !(1 << ix);
                }
                !(*state == (1 << ins.len()) - 1)
            }
        };

        for new_target in targets {
            q.push_back((target.clone(), new_pulse, new_target.to_string()));
        }
    }

    (count[0], count[1], high_from)
}

fn main() {
    let mod_re = Regex::new(r"([%&]?)(.*) -> (.*)").unwrap();
    let mut inputs = HashMap::new();
    let mut init = HashMap::new();
    for line in lines() {
        let (_, [typ, name, targets]) =
            mod_re.captures(&line).unwrap().extract();
        let targets = targets.split(", ").map_v(str::to_string);
        for t in &targets {
            inputs
                .entry(t.clone())
                .or_insert(Vec::new())
                .push(name.to_string());
        }
        init.insert(name.to_string(), (Mod::new(typ), targets));
    }

    let mut state = init.clone();
    let (mut low, mut high) = (0, 0);
    for _ in 0..1000 {
        let (l, h, _) = push_button(&inputs, &mut state, "");
        low += l;
        high += h;
    }
    println!("{}", low * high);

    if !inputs.contains_key("rx") {
        return; // for example input, end with part 1
    }

    let pre_rx = &inputs["rx"][0];
    let mut cycles = vec![0u64; inputs[pre_rx].len()];
    let mut found = 0;
    let mut round = 0;
    state = init;

    while found < cycles.len() {
        round += 1;
        if let (_, _, Some(hf)) = push_button(&inputs, &mut state, pre_rx) {
            if cycles[hf] == 0 {
                cycles[hf] = round;
                found += 1;
            }
        }
    }
    println!("{}", cycles.into_iter().product_());
}
