use std::io;

// Do I really need to write the full type here, including the number
// of elements? Sure I do. Truly empowering…
// The fascinating thing is that the error message tells me the exact type.
// So it seems that Rust is able to do type deduction, it just flatly refuses
// to actually use it. /o\
// (Note that even K&R C can infer the size of an initialised array.)
// Also, having a global Vec initialised to some values seems completely
// impossible.
const DIGITS: [(&str, i32); 18] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

fn get_digits(line: &str) -> (i32, i32) {
    let mut first = -1;
    let mut last = -1;
    for i in 0..line.len() {
        for (pat, val) in DIGITS {
            // No & with method calls; here it is implicit.
            if line[i..].starts_with(pat) {
                if first == -1 {
                    first = val
                }
                last = val
            }
        }
    }
    (first, last)
}

fn main() {
    let sum: i32 = io::stdin()
        .lines()
        .map(|line| {
            let (a, b) = get_digits(&line.unwrap());
            a * 10 + b
        })
        .sum();

    println!("{}", sum);
}
