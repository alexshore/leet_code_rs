fn main() {
    println!(
        "{:?}",
        maximum_wealth(vec![vec![1, 5], vec![7, 3], vec![3, 5]])
    );
}

fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    accounts
        .iter()
        .map(|account| account.iter().sum())
        .max()
        .unwrap()
}
