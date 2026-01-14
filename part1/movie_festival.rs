use std::io;

fn main() {
    let mut n = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut n);

    let movie_amount = n.split_whitespace()
                        .next()
                        .expect("")
                        .parse::<i64>()
                        .unwrap();

    let mut movies: Vec<(i64, i64)> = Vec::new();
    for _ in 0..movie_amount {
        let mut movie = String::new();
        let _ = stdin.read_line(&mut movie);

        let mut times = movie.split_whitespace();
        let start = times.next()
                         .expect("")
                         .parse::<i64>()
                         .unwrap();
        let end = times.next()
                       .expect("")
                       .parse::<i64>()
                       .unwrap();

        movies.push((start, end));
    }

    movies.sort();

    let mut previous_ending = 0;
    let mut current_ending = 0;
    let mut movies_watched = 0;

    for movie in movies {
        let movie_starting = movie.0;
        let movie_ending = movie.1;

        if current_ending <= movie_starting {
            previous_ending = current_ending;
            current_ending = movie_ending;
            movies_watched += 1;
            continue;
        }
                                            // dunno if needed
        if current_ending > movie_ending && previous_ending <= movie_starting {
            current_ending = movie_ending;
            continue;
        }
    }
    
    println!("{}", movies_watched);

}

