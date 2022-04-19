// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

// #[derive(Debug)]
enum Ticket {
    Backstage(f64, String),
    Vip(f64, String),
    Standard(f64),
}

fn main() {
    let tickets = vec![
        Ticket::Backstage(12.3, "Mohamed".to_owned()),
        Ticket::Vip(22.1, String::from("Bamoh")),
        Ticket::Standard(6.2),
    ];

    for t in tickets {
        match t {
            Ticket::Standard(p) => println!("Standard ticket with price {:?}", p),
            Ticket::Vip(p, n) => println!("{:?} VIP ticket {:?}", n, p),
            Ticket::Backstage(p, n) => println!("{:?} Backstage ticket {:?}", p, n),
        }
    }
}
