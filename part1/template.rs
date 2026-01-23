use std::io;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut buffer);

    let largest = buffer.split_whitespace()
                        .next()
                        .expect("")
                        .parse::<i64>()
                        .unwrap();

}

