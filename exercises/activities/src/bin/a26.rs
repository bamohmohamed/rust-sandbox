// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats

use chrono::{Local, DateTime};

fn main() {
    let dt: DateTime<Local> = Local::now();
    println!("{}",dt.format("%a %b %e %T %Y").to_string())

}
