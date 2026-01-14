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

    let ispossible = largest % 4 == 3 || largest % 4 == 0;

    if !ispossible {
        println!("NO");

    } else {
        println!("YES");

        let mut set1: Vec<i64> = Vec::new();
        let mut set1sum = 0;
        let mut set1str = String::new();
        let mut set2: Vec<i64> = Vec::new();
        let mut set2sum = 0;
        let mut set2str = String::new();

        for i in (1..largest+1).rev() {
            if set1sum <= set2sum {
                set1.push(i);
                set1sum += i;
            } else {
                set2.push(i);
                set2sum += i;
            }
        }

        for i in &set1 {
            set1str.push_str(&i.to_string());
            set1str.push_str(" ");
        }
        set1str = set1str[..set1str.len()-1]
                    .to_string();

        for i in &set2 {
            set2str.push_str(&i.to_string());
            set2str.push_str(" ");
        }
        set2str = set2str[..set2str.len()-1]
                    .to_string();
    
        println!("{}", set1.len());
        println!("{}", set1str);
        println!("{}", set2.len());
        println!("{}", set2str);
    }
}

