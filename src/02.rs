use std::cmp::max;
use std::io;
use std::str::FromStr;

// I really miss auto return type deduction.
fn lines() -> impl Iterator<Item = String> {
    io::stdin().lines().map(|l| l.unwrap())
}

// Useful methods so we can avoid all that unwrapping and collecting.
trait IteratorPlus: Iterator {
    fn map_v<B, F>(self, f: F) -> Vec<B>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B,
    {
        self.map(f).collect()
    }

    fn next_(&mut self) -> Self::Item {
        self.next().unwrap()
    }

    fn nth_(&mut self, n: usize) -> Self::Item {
        self.nth(n).unwrap()
    }
}

impl<T: Iterator> IteratorPlus for T {}

trait StringPlus: Sized {
    fn parse_<F: FromStr>(&self) -> F;
}

impl StringPlus for &str {
    fn parse_<F: FromStr>(&self) -> F {
        self.parse().ok().unwrap()
    }
}

#[derive(Copy, Clone, Default)]
struct Cubes {
    red: i32,
    green: i32,
    blue: i32,
}

fn main() {
    let games = lines().map_v(|line| {
        line.split(": ").nth_(1).split("; ").map_v(|game| {
            let mut cs = Cubes::default();
            game.split(", ").for_each(|cube| {
                let mut it = cube.split(' ');
                let n = it.next_().parse_::<i32>();
                let c = it.next_();
                match c {
                    "red" => cs.red += n,
                    "green" => cs.green += n,
                    "blue" => cs.blue += n,
                    _ => panic!(),
                }
            });
            cs
        })
    });

    let limit = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };
    let mut part1 = 0;
    for (game, i) in games.iter().zip(1..) {
        if game.iter().all(|cube|
            cube.red <= limit.red &&
            cube.green <= limit.green &&
            cube.blue <= limit.blue
        ) {
            part1 += i;
        }
    }

    let mut part2 = 0;
    for game in games.iter() {
        // Using ‹game.iter().reduce(…)› doesn't work, because the
        // iterated-over elements are references, and the lambda thus cannot
        // produce new elements by value.
        //
        // One solution is to use ‹game.iter().cloned().reduce(…)›.
        // Alternatively, we can write an explicit for cycle or use ‹fold›
        // with an initial value.
        let m = game.iter().fold(Cubes::default(), |a, b| Cubes {
            red: max(a.red, b.red),
            green: max(a.green, b.green),
            blue: max(a.blue, b.blue),
        });

        part2 += m.red * m.green * m.blue;
    }

    println!("{}", part1);
    println!("{}", part2);
}
