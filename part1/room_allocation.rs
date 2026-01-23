use std::io;
use std::collections::BinaryHeap;

fn main() {
    let mut n = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut n);

    let largest = n.split_whitespace()
                   .next()
                   .expect("")
                   .parse::<i64>()
                   .unwrap();

    let mut customers = Vec::new();
    for i in 0..largest {
        let mut c = String::new();
        let _ = stdin.read_line(&mut c);

        let mut c_split = c.split_whitespace();
        let c1 = c_split.next()
                        .expect("")
                        .parse::<i64>()
                        .unwrap();

        let c2 = c_split.next()
                        .expect("")
                        .parse::<i64>()
                        .unwrap();

        customers.push((c1, c2, i));
    }

    customers.sort();

    let mut customer_rooms: Vec<(i64, i64)> = Vec::new();
    let mut room_reservations: BinaryHeap<(i64, i64)> = BinaryHeap::new();
    let mut room_amount = 0;
    let mut current_room_number = 1;

    for customer in customers {
        match room_reservations.pop() {
            Some(t) => {
                if -t.0 < customer.0 {
                    room_reservations.push((-customer.1, t.1));
                    customer_rooms.push((t.1, customer.2));
                } else {
                    room_amount += 1;
                    room_reservations.push((t.0, t.1));
                    room_reservations.push((-customer.1, current_room_number));
                    customer_rooms.push((current_room_number, customer.2));
                    current_room_number += 1;
                }
            }
            None => {
                room_amount += 1;
                room_reservations.push((-customer.1, current_room_number));
                customer_rooms.push((current_room_number, customer.2));
                current_room_number += 1;
            }
        }
    }

    customer_rooms.sort_by_key(|k| k.1);

    println!("{}", room_amount);
    for customer in customer_rooms {
        print!("{} ", customer.0);
    }
    println!("");
}

