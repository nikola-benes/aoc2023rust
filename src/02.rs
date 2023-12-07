use aoc::*;
use std::cmp::max;

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
        let m = game.iter().fold1_(|a, b| Cubes {
            red: max(a.red, b.red),
            green: max(a.green, b.green),
            blue: max(a.blue, b.blue),
        });

        part2 += m.red * m.green * m.blue;
    }

    println!("{}", part1);
    println!("{}", part2);
}
