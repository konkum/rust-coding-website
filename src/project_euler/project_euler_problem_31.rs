type Pence = u8;

const COINS: [Pence; 8] = [200, 100, 50, 20, 10, 5, 2, 1];

pub fn coin_sums(target: Pence, max_coin: usize) -> usize {
    return (max_coin..8)
        .map(|i| {
            let coin = COINS[i];

            if coin == target {
                return 1;
            } else if coin <= target {
                return coin_sums(target - coin, i);
            } else {
                return 0;
            }
        })
        .sum();
}
