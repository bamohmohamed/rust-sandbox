// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn print_result(b: bool) {
    match b {
        true => println!("its big"),
        _ => println!("its small"),
    }
}

fn main() {
    let num = 120;
    let is_greater_than100 = if num > 100 { true } else { false };
    print_result(is_greater_than100);
}
