use std::io;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut buffer);

    let mut nk = buffer.split_whitespace();

    let _n = nk.next()
              .expect("")
              .parse::<i64>()
              .unwrap();

    let k = nk.next()
              .expect("")
              .parse::<i64>()
              .unwrap();

    let _ = stdin.read_line(&mut buffer);
    let nums = buffer.split_whitespace();

    let mut numbers = Vec::new();
    for num in nums {
        let number = num.parse::<i64>()
                        .unwrap();
        numbers.push(number);
    }

    // mä tarviin lisää esimerkkei
    
}

