// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> bool {
    n >= lower && n <= upper
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b != 0 {
        Some(a / b)
    } else { None }
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::{clamp, concat, div};

    #[test]
    fn check_clamp() {
        assert_eq!(clamp(6, 5, 7), true, "Result should be 6> 5 and 6 < 7")
    }

    #[test]
    fn check_div() {
        assert_eq!(div(16, 4), Some(4), "16 divided by 4 should be 3")
    }

    #[test]
    fn check_div_zero() {
        assert_eq!(div(16, 0), None, "Div by Zero should be None")
    }

    #[test]
    fn check_concat() {
        assert_eq!(concat("Mo", "Bamoh"), String::from("MoBamoh"), "Result should be MoBamoh")
    }
}