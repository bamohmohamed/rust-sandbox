// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

struct Dimension {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Color {
    BLUE,
    GREEN,
}
struct Box {
    dimension: Dimension,
    weight: f64,
    color: Color,
}
impl Box {
    fn new_box(dimension: Dimension, weight: f64, color: Color) -> Self {
        Self {
            dimension,
            weight,
            color,
        }
    }

    fn print_box(&self) {
        println!(
            "Box: Dimension {:?}x{:?} - Weight {:?} - Color {:?}",
            self.dimension.x, self.dimension.y, self.weight, self.color
        )
    }
}

fn main() {
    let b = Box::new_box(Dimension { x: 2, y: 5 }, 13.1, Color::BLUE);
    b.print_box();

    let b1 = Box::new_box(Dimension { x: 4, y: 9 }, 21.3, Color::GREEN);
    b1.print_box()
}
