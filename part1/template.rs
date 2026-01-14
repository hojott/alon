use std::io;

fn main() {
    let mut n = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut n);

    let largest = n.split_whitespace()
                   .next()
                   .expect("")
                   .parse::<i64>()
                   .unwrap();

}

