use std::io;

fn main() {
    let stdin = io::stdin();
    let mut n = String::new();
    let _ = stdin.read_line(&mut n);
    let _coin_amount = n.split_whitespace()
                       .next()
                       .expect("")
                       .parse::<i64>()
                       .unwrap();

    let mut coin_string = String::new();
    let _ = stdin.read_line(&mut coin_string);
    let coins_split = coin_string.split_whitespace();

    let mut coins = Vec::new();
    for coin in coins_split {
        coins.push(coin.parse::<i64>().unwrap());
    }

    coins.sort();
    
    let mut sum = 1;
    while sum <= 1_000_000_000 {
        if find_sum(sum, &coins) {
            sum += 1
        } else {
            break
        }
    }

    println!("{}", sum);
}


fn find_sum(sum: i64, coins: &Vec<i64>) -> bool {
    if sum == 0 { return true }
    if sum < 0 { return false }

    for (i, coin) in coins.into_iter().enumerate() {
        if *coin > sum { return false }
        if *coin == 0 { continue }

        let mut coins_clone = coins.clone();
        coins_clone[i] = 0;

        if find_sum(sum-coin, &coins_clone) { return true }
    }

    return false

}

