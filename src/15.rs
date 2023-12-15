use aoc::*;

fn hash(s: &str) -> usize {
    s.bytes()
        .fold(0u8, |h, b| h.wrapping_add(b).wrapping_mul(17)) as usize
}

fn main() {
    let input = lines().next_();
    let ops = input.split(',');
    println!("{}", ops.clone().map(hash).sum_());

    let mut boxes = vec![Vec::<(&str, i32)>::new(); 256];
    for op in ops {
        let (label, fl) = op.split(['-', '=']).to_pair();
        let box_ = &mut boxes[hash(label)];
        let ix = box_.position(|&(x, _)| x == label);

        if fl.is_empty() {
            ix.map(|i| box_.remove(i));
            continue;
        }

        let n = fl.parse_();
        if let Some(i) = ix {
            box_[i].1 = n;
        } else {
            box_.push((label, n));
        }
    }

    println!(
        "{}",
        boxes
            .zip(1..)
            .map(|(b, m)| b.zip(1..).map(move |((_, fl), p)| fl * p * m))
            .flatten()
            .sum_()
    );
}
