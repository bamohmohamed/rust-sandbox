// Topic: Map combinator
//
// Requirements:
// * Given a user name, create and print out a User struct if the user exists
//
// Notes:
// * Use the existing find_user function to locate a user
// * Use the map function to create the User
// * Print out the User struct if found, or a "not found" message if not

#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

impl User{
    fn new(user_id:i32,name:String)->Self {
        Self{user_id,name}
    }
}
/// Locates a user id based on the name.
fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "katie" => Some(9),
        _ => None,
    }
}

fn main() {

    let name ="mo";

    let some_user = find_user(&name).map(|id| User::new(id,String::from(name)));
    match some_user {
        Some(user) => println!("{:?}",user),
        None => println!("user not found")
    }

}
