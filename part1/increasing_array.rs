use std::io;

fn main() {
    let mut size = String::new();
    let mut data = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut size);
    let _ = stdin.read_line(&mut data);

    let datasplit = data.split_whitespace();
    let mut largest: Option<i64> = None;
    let mut count: i64 = 0;
    for unparsed in datasplit {
        let num = unparsed.parse::<i64>()
                          .unwrap();
        match largest {
            Some(l) => {
                if l >= num { count += l - num }
                else { largest = Some(num) }
            }
            None    => { largest = Some(num) }
        }
    }

    println!("{}", count)
}