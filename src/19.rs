use aoc::*;
use once_cell::sync::Lazy;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};

enum Rule {
    Goto(String),
    If(usize, Ordering, i64, String),
}

fn var_to_ix(var: &str) -> usize {
    match var {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => panic!(),
    }
}

fn parse_rule(rule: &str) -> Rule {
    static RULE_RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(.*)([<>])(.*):(.*)|(.*)").unwrap());
    let cap = RULE_RE.captures(rule).unwrap();

    if let Some(target) = cap.get(5) {
        // :empowered:
        return Rule::Goto(target.as_str().to_string());
    }

    // Captures cannot use slices.
    // Also, for some reason, cap[i] seems to return str, not &str.
    let (var, op, imm, target) = (&cap[1], &cap[2], &cap[3], &cap[4]);
    Rule::If(
        var_to_ix(var),
        if op == "<" {
            Ordering::Less
        } else {
            Ordering::Greater
        },
        imm.parse_(),
        target.to_string(),
    )
}

fn accepted(wfs: &HashMap<String, Vec<Rule>>, part: &[i64; 4]) -> bool {
    let mut wf = "in";
    while wf != "A" && wf != "R" {
        for rule in &wfs[wf] {
            match rule {
                Rule::Goto(next) => {
                    wf = next;
                    break;
                }
                Rule::If(var, ord, imm, target) => {
                    if part[*var].cmp(imm) == *ord {
                        wf = target;
                        break;
                    }
                }
            }
        }
    }
    wf == "A"
}

fn split_parts(
    parts: &mut [(i64, i64); 4],
    var: usize,
    ord: Ordering,
    imm: i64,
) -> Option<[(i64, i64); 4]> {
    let mut new = *parts;
    if ord == Ordering::Less {
        if parts[var].0 >= imm {
            return None;
        }
        new[var].1 = imm - 1;
        parts[var].0 = imm;
    } else {
        if parts[var].1 <= imm {
            return None;
        }
        new[var].0 = imm + 1;
        parts[var].1 = imm;
    }
    Some(new)
}

fn main() {
    let wf_re = Regex::new(r"(.*)\{(.*)\}").unwrap();
    let part_re = Regex::new(r"\{x=(.*),m=(.*),a=(.*),s=(.*)\}").unwrap();

    let mut wfs = HashMap::new();

    for line in lines() {
        if line.is_empty() {
            break;
        }
        let (_, [name, rules]) = wf_re.captures(&line).unwrap().extract();
        wfs.insert(name.to_string(), rules.split(',').map_v(parse_rule));
    }

    println!(
        "{}",
        lines()
            .filter_map(|line| {
                let (_, part): (_, [_; 4]) =
                    part_re.captures(&line).unwrap().extract();
                let part = part.map(|p| p.parse_());
                accepted(&wfs, &part).then_some(part.into_iter().sum_())
            })
            .sum_()
    );

    let mut part2 = 0;
    let mut q = VecDeque::new();
    let start = [(1, 4000); 4];
    q.push_back((start, "in"));

    while let Some((mut parts, wf)) = q.pop_front() {
        if wf == "R" {
            continue;
        }
        if wf == "A" {
            part2 += parts.iter().map(|(from, to)| to - from + 1).product_();
            continue;
        }
        for rule in &wfs[wf] {
            match rule {
                Rule::Goto(target) => {
                    q.push_back((parts, target));
                    break;
                }
                Rule::If(var, ord, imm, target) => {
                    if let Some(new_parts) =
                        split_parts(&mut parts, *var, *ord, *imm)
                    {
                        q.push_back((new_parts, target));
                    }
                }
            }
        }
    }
    println!("{}", part2);
}
