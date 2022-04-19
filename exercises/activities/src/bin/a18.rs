// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Customer {
    name: String,
    age: i32,
}

impl Customer {
    fn new(name: String, age: i32) -> Self {
        Self { name, age }
    }
}

fn can_purchase(c: &Customer) -> Result<bool, String> {
    if c.age < 21 {
        Err("customer under age of 21!".to_owned())
    } else {
        Ok(true)
    }
}

fn main() {
    let c1 = Customer::new("Mohamed".to_owned(), 32);
    let c2 = Customer::new("Ghita".to_owned(), 5);

    let r = can_purchase(&c1);
    println!("{:?} {:?}", c1.name, r);
    let r2 = can_purchase(&c2);
    println!("{:?} {:?}", c2.name, r2);
}
