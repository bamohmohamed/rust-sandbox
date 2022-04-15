// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    GINGER,
    JASMINE,
    PEACH,
}

#[derive(Debug)]
enum Type {
    WATER,
    TEA,
    COFFEE,
}
struct Drink {
    typ: Type,
    flavor: Flavor,
    fluid_ounces: f64,
}

fn print_drink_flavor(drink: Drink) {
    match drink.flavor {
        Flavor::GINGER => println!(
            "{:?} oz {:?} with ginger flavour",
            drink.fluid_ounces, drink.typ
        ),
        Flavor::JASMINE => println!(
            "{:?} oz {:?} with jasmine flavour",
            drink.fluid_ounces, drink.typ
        ),
        Flavor::PEACH => println!(
            "{:?} oz {:?} with peach flavour ",
            drink.fluid_ounces, drink.typ
        ),
    }
}

fn main() {
    let ginger_tea = Drink {
        typ: Type::TEA,
        flavor: Flavor::GINGER,
        fluid_ounces: 75.0,
    };

    let jasmine_water = Drink {
        typ: Type::WATER,
        flavor: Flavor::JASMINE,
        fluid_ounces: 50.0,
    };

    let peach_coffee = Drink {
        typ: Type::COFFEE,
        flavor: Flavor::PEACH,
        fluid_ounces: 25.0,
    };

    print_drink_flavor(ginger_tea);
    print_drink_flavor(jasmine_water);
    print_drink_flavor(peach_coffee);
}
