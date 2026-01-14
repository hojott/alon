use std::io;
use std::convert::TryInto;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut buffer);
    /*let initial = buffer.chars()
                        .nth(0)
                        .unwrap()
                        .to_digit(10)
                        .unwrap();*/
    let initial = buffer[..buffer.len()-1].parse::<i64>()
                                          .unwrap()
                                          .try_into()
                                          .unwrap();
    let mut line: Vec<i64> = Vec::new();
    line.push(initial);
    txpo(initial, &mut line);

    let mut string = String::new();
    /*let _ = line.into_iter()
                .map(|num| {
                    string.push_str(&(num.to_string()));
                    string.push_str("");
                });*/
    for num in &line {
        string.push_str(&num.to_string());
        if num != &1 { string.push_str(" ") }
    }
    
    println!("{}", string);
}

fn txpo(num: i64, line: &mut Vec<i64>) {
    let mut current = num;
    loop {
        if current == 1 { break }
        else if current % 2 == 0 { line.push(current/2) }
        else { line.push(3*current + 1) }
        
        current = line[line.len()-1];
    }
}
