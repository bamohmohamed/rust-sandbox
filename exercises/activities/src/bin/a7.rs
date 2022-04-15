// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
    GREEN,
    BLUE,
    YELLOW,
    BLACK,
    WHITE,
}

fn print_color_name(color: Color) {
    match color {
        Color::WHITE => println!("white"),
        Color::BLACK => println!("black"),
        Color::BLUE => println!("blue"),
        Color::GREEN => println!("green"),
        Color::YELLOW => println!("yellow"),
    }
}

fn main() {
    print_color_name(Color::BLUE);
    print_color_name(Color::YELLOW);
    print_color_name(Color::WHITE);
    print_color_name(Color::BLACK);
    print_color_name(Color::GREEN);
}
