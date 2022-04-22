// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;


struct Item {
    name: String,
    qty: i32,
}

impl Item {
    fn new(name: String, qty: i32) -> Self {
        Self { name, qty }
    }
}

fn main() {
    let mut store = HashMap::new();
    store.insert("sku1", Item::new("Chairs".to_owned(), 5));
    store.insert("sku2", Item::new("Beds".to_owned(), 3));
    store.insert("sku3", Item::new("Tables".to_owned(), 2));
    store.insert("sku4", Item::new("Couches".to_owned(), 0));

    let mut count = 0;
    for (_, item) in store {
        let res = if item.qty <= 0 {
            format!("{:?} out of stock", item.name)
        } else {
            count = count + item.qty;
            format!("{:?} in stock {:?}", item.name, item.qty)
        };
        println!("{:?}", res);
    }
    println!("Total items in stock is {:?}",count);
}
