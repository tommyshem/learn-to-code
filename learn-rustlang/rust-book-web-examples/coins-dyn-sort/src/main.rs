fn main() {
    let mut usage = [0; 8];
    let coins = [1, 5, 10, 25, 50];
    for i in 0..usage.len() {
        let amount = i + 1;
        if amount < coins[0] {
            continue;
        }
        if coins.contains(&amount) {
            usage[i] = 1;
            continue;
        }
        let mut min = 1000000;
        for j in 0..coins.len() {
            if coins[j] > amount {
                break;
            }
            let potent = usage[i - coins[j]] + 1;
            if potent < min {
                min = potent
            }
        }
        usage[i] = min
    }
    println!(
        "Optimal number of coins for {} is {}",
        usage.len(),
        usage[usage.len() - 1]
    )
}
