use std::io;
use std::collections::HashMap;

fn main() {
    let mut largest_bf = String::new();
    let mut numbers = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut largest_bf);
    let _ = stdin.read_line(&mut numbers);

    /*let largest = largest_bf.split_whitespace()
                            .next()
                            .expect("")
                            .parse::<i64>()
                            .unwrap();
    */
    let shuffled = numbers.split_whitespace();
    let mut found = HashMap::new();
    let mut i = 0;

    for raw_num in shuffled {
        let num = raw_num.parse::<i64>()
                         .unwrap();

        if !found.contains_key(&(num-1)) {
            i += 1;
        }
        found.insert(num, 0);
    }


    println!("{}", i);
}

