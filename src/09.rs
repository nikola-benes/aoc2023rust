use aoc::*;

fn prev_value(mut nums: Vec<i64>) -> i64 {
    nums.reverse();
    next_value(nums)
}

fn next_value(nums: Vec<i64>) -> i64 {
    if nums.iter().all(|&n| n == 0) {
        return 0;
    }
    nums.last().unwrap()
        + next_value(
            nums.iter().zip(nums.iter().skip(1)).map_v(|(&a, &b)| b - a),
        )
}

fn main() {
    let input =
        lines().map_v(|line| line.split_whitespace().map_v(|n| n.parse_()));

    println!("{}", input.iter().cloned().map(next_value).sum_());
    println!("{}", input.into_iter().map(prev_value).sum_());
}
