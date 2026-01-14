use std::io;

fn main() {
    let mut n_raw = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut n_raw);

    let n = n_raw.split_whitespace()
                   .next()
                   .expect("")
                   .parse::<usize>()
                   .unwrap();

    let mut series: Vec<usize> = vec![0;n];
    for i in 1..n+1 {
        let mut spessu = 0;
        let mut v = 0;

        for j in 1..18 {
            if spessu == 0 && !((n+1) % 2_usize.pow(j-1) == 0) {
                spessu = 2_usize.pow(j-1);
            }

            println!("n: {}, i: {}, j: {}, v: {}, s: {}", n, i, j, v, spessu);
            if (i + 1+2_usize.pow(j-1) - spessu) % 2_usize.pow(j) == 0 {
                if j == 1 {
                    v += i/2_usize.pow(j);
                } else {
                    v += (i + 1+2_usize.pow(j-1) - spessu)/2_usize.pow(j);
                }
                println!("(n: {}, i: {}, j: {}, v: {}, s: {})", n, i, j, v, spessu);
                break;
            } else {
                if j == 1 {
                    v += n/2_usize.pow(j);
                } else {
                    v += 1 + (n - 2_usize.pow(j) + 1+2_usize.pow(j-1)-spessu)/2_usize.pow(j);
                }
            }
        }
        
        if v > series.len() {
            panic!("RAAAH")
        }
        series[v-1] = i;
    }
    
    let mut vstr = String::new();
    for (i, v) in series.into_iter().enumerate() {
        if i != 0 {
            vstr += " ";
        }
        vstr += &v.to_string();
    }

    println!("{}", vstr);
}

