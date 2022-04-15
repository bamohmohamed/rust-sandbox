// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

struct Coordinate {
    x: i32,
    y: i32,
}

fn coordinate(coor: Coordinate) -> (i32, i32) {
    (coor.x, coor.y)
}

fn main() {
    let coor = Coordinate { x: 6, y: 8 };
    let (_x, y) = coordinate(coor);
    if y > 5 {
        println!(">5");
    } else if y < 5 {
        println!("<5");
    } else {
        println!("=5");
    }
}
