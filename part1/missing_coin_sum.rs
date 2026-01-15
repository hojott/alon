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
 
    let mut sum = 0;
    for coin in &coins {
        if *coin > sum + 1 {
            break
        } else {
            sum += coin;
        }
    }

    sum += 1;

    println!("{}", sum);
}

