// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Item {
    id: i32,
    qty: i32,
}

fn display_qty(item: &Item) {
    println!("Quantity is {:?}", item.qty);
}

fn display_id(item: &Item) {
    println!("Id is {:?}", item.id);
}
fn main() {
    let item = Item { id: 15, qty: 6 };
    display_id(&item); //borrow don't move
    display_qty(&item);
}
