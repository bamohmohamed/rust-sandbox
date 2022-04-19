// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    name: String,
    age: i32,
    fav_color: String,
}
impl Person {
    fn new(name: String, age: i32, fav_color: String) -> Self {
        Self {
            name,
            age,
            fav_color,
        }
    }
}

fn print_name(name: &str) {
    println!("Name is {:?}", name);
}

fn print_color(color: &str) {
    println!("Favorite color is {:?}", color);
}

fn main() {
    let people = vec![
        Person::new("Fati".to_owned(), 6, "GOLD".to_owned()),
        Person::new("Ghita".to_owned(), 3, "RED".to_owned()),
        Person::new("Yakout".to_owned(), 2, "PINK".to_owned()),
    ];

    for person in people {
        if person.age <= 10 {
            print_name(&person.name);
            print_color(&person.fav_color);
        }
    }
}
