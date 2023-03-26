fn main() {
    println!("{:?}", running_sum(vec![3, 1, 2, 10, 1]));
}

fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
    for i in 1..nums.len() {
        nums[i] += nums[i - 1];
    }
    nums
}
