use aoc::*;
use z3::ast::Ast;

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

    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);

    let [rx, ry, rz, vrx, vry, vrz] = ["rx", "ry", "rz", "vrx", "vry", "vrz"]
        .map(|v| z3::ast::Real::new_const(&ctx, v));

    for (i, &((x, y, z), (vx, vy, vz))) in hailstones.enumerate() {
        let t_i = z3::ast::Real::new_const(&ctx, format!("t{}", i));
        for (c, vc, rc, vrc) in
            [(x, vx, &rx, &vrx), (y, vy, &ry, &vry), (z, vz, &rz, &vrz)]
        {
            let [c, vc] = [c, vc]
                .map(|v| z3::ast::Int::from_i64(&ctx, v as i64).to_real());
            solver.assert(&(c + &t_i * vc)._eq(&(rc + &t_i * vrc)));
        }
    }

    solver.check();
    let model = solver.get_model().unwrap();

    println!(
        "{}",
        model
            .eval(&(rx.to_int() + ry.to_int() + rz.to_int()), true)
            .unwrap()
    );
}
