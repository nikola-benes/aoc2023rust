use std::io;

fn main() {
    // If I write no type here, Rust protests.
    // If I write i32 here, Rust protests and tells me to write u32 here.
    // What an empowering language!
    let sum: u32 = io::stdin()
        .lines()
        .map(|line| {
            let v: Vec<_> = line
                .unwrap()
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect();
            v[0] * 10 + v[v.len() - 1]
        })
        .sum();

    println!("{}", sum);
}
