use aoc::*;

fn prev_value(mut nums: Vec<i64>) -> i64 {
    nums.reverse();
    next_value(nums)
}

fn next_value(mut nums: Vec<i64>) -> i64 {
    next_value_impl(nums.as_mut_slice());
    nums.into_iter().sum_()
}

fn next_value_impl(nums: &mut [i64]) {
    if nums.all(|&n| n == 0) {
        return;
    }
    let last = nums.len() - 1;
    for i in 0..last {
        nums[i] = nums[i + 1] - nums[i];
    }
    next_value_impl(&mut nums[..last]);
}

fn main() {
    let input =
        lines().map_v(|line| line.split_whitespace().map_v(|n| n.parse_()));

    println!("{}", input.iter().cloned().map(next_value).sum_());
    println!("{}", input.into_iter().map(prev_value).sum_());
}
