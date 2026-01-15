use std::io;

fn main() {
    let stdin = io::stdin();

    let mut _ns = String::new();
    let _ = stdin.read_line(&mut _ns);
    
    // O(n)
    let mut ticket_str = String::new();
    let _ = stdin.read_line(&mut ticket_str);
    let ticket_split = ticket_str.split_whitespace();
    let mut tickets = Vec::new();
    for ticket in ticket_split {
        tickets.push(ticket.parse::<i64>().unwrap());
    }
    
    // O(n)
    let mut customer_str = String::new();
    let _ = stdin.read_line(&mut customer_str);
    let customer_split = customer_str.split_whitespace();
    let mut customers = Vec::new();
    for customer in customer_split {
        customers.push(customer.parse::<i64>().unwrap());
    }

    // O(nlogn)
    tickets.sort();

    // O(nlogn) -- O(n)
    let mut max_loop = 0;
    for customer in customers {
        if tickets.len() == 0 {
            println!("{}", -1);
            break
        }

        let mut start = 0;
        let mut end = tickets.len()-1;
        let mut i: usize;
        let mut loop_num = 0;
        // -- O(logn)
        loop {
            loop_num += 1;
            if loop_num > max_loop {
                max_loop = loop_num;
            }
            i = start+(end-start)/2;
            if customer == tickets[i] {
                println!("{}", tickets[i]);
                tickets.drain(i..i+1);
                break
            } else if customer > tickets[i] && start != end {
                start = i+1;
                continue
            } else if customer < tickets[i] && start != end {
                end = i;
                continue
            } else if customer > tickets[i] && start == end {
                println!("{}", tickets[i]);
                tickets.drain(i..i+1);
                break
            } else if customer < tickets[i] && start == end {
                if i > 0 && customer > tickets[i-1] {
                    println!("{}", tickets[i-1]);
                    tickets.drain(i-1..i);
                    break
                } else {
                    println!("{}", -1);
                    break
                }
            }
        }
    }
}

