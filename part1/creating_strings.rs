use std::io;

fn main() {
    let mut original = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut original);

    let chars: Vec<char> = original[..original.len()-1]
                                   .chars()
                                   .collect();

    let mut permutations = permutate(&chars);

    permutations.sort();

    permutations.dedup();

    println!("{}", permutations.len());
    for letters in &permutations {
        let word: String = letters.into_iter().collect();

        println!("{}", word);
    }
}

fn permutate(og: &[char]) -> Vec<Vec<char>> {
    let n = og.len();
    //println!("{}", n);
    let mut permutations: Vec<Vec<char>> = Vec::new();

    if n == 1 { return vec![og.to_vec()] }

    if n == 2 {
        //println!("aaaah");
        permutations.push([og[0], og[1]].to_vec());
        permutations.push([og[1], og[0]].to_vec());
        //println!("word: {}", permutations[0].clone().into_iter().collect::<String>());
        //println!("len: {}", permutations[0].len());
        //println!("word: {}", permutations[1].clone().into_iter().collect::<String>());
        //println!("len: {}", permutations[1].len());

        return permutations
    }

    for i in 0..n {
        let mut new_vec = og.to_vec();
        (new_vec[0], new_vec[i]) = (new_vec[i], new_vec[0]);
        //println!("{}", &new_vec[..].into_iter().collect::<String>());

        let subperms = permutate(&new_vec[1..]);
        //println!("rawr");

        for perm in subperms {
            let mut new_new_vec = [new_vec[0]].to_vec();
            new_new_vec.extend_from_slice(&perm[..]);
            //println!("{}", perm.into_iter().collect::<String>());
            permutations.push(new_new_vec);
        }
    }

    return permutations
}
