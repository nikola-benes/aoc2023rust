use std::cmp::max;
use std::io;

// I really miss auto return type deduction.
fn lines() -> impl Iterator<Item = String> {
    io::stdin().lines().map(|l| l.unwrap())
}

#[derive(Copy, Clone, Default)]
struct Cubes {
    red: i32,
    green: i32,
    blue: i32,
}

fn main() {
    let games = lines()
        .map(|line| {
            line.split(": ")
                .nth(1)
                .unwrap()
                .split("; ")
                .map(|game| {
                    let mut cs = Cubes::default();
                    game.split(", ").for_each(|cube| {
                        let mut it = cube.split(' ');
                        // Christmas comes early this year!
                        // So much unwrapping…
                        // I would love to use ‹and_then› here, but
                        // Option and Result cannot be combined that way.
                        let n = it.next().unwrap().parse::<i32>().unwrap();
                        let c = it.next().unwrap();
                        match c {
                            "red" => cs.red += n,
                            "green" => cs.green += n,
                            "blue" => cs.blue += n,
                            _ => panic!(),
                        }
                    });
                    cs
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // Perhaps a map_collect combo would be nice.

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
