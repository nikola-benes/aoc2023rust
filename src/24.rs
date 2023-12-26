use aoc::*;

type Hailstone = ((i128, i128, i128), (i128, i128, i128));

fn sgn(num: i128) -> i128 {
    match num {
        0 => 0,
        1.. => 1,
        _ => -1,
    }
}

fn intersect_in(a: &Hailstone, b: &Hailstone, (lo, hi): (i128, i128)) -> bool {
    let ((ax, ay, _), (vax, vay, _)) = a;
    let ((bx, by, _), (vbx, vby, _)) = b;
    let det = vbx * vay - vby * vax;
    let s = sgn(det);

    let (dx, dy) = (bx - ax, by - ay);
    let det_a = vbx * dy - vby * dx;
    let det_b = vax * dy - vay * dx;

    det != 0
        && sgn(det_a) != -s
        && sgn(det_b) != -s
        && (lo - ax) * det * s <= vax * det_a * s
        && (hi - ax) * det * s >= vax * det_a * s
        && (lo - ay) * det * s <= vay * det_a * s
        && (hi - ay) * det * s >= vay * det_a * s
}

fn main() {
    let hailstones = lines().map_v(|line| {
        let mut it = line.split([',', '@']).map(|n| n.trim().parse_());
        (it.to_triple(), it.to_triple())
    });

    let bounds = if hailstones.len() == 5 {
        (7, 27) // small input
    } else {
        (200000000000000, 400000000000000)
    };

    println!(
        "{}",
        hailstones
            .enumerate()
            .map(|(i, a)| hailstones[..i].map(|b| intersect_in(a, b, bounds)))
            .flatten()
            .filter(|&x| x)
            .count()
    );
}
