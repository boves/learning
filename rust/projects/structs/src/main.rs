fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    println!("user1.email is: {}", user1.email);
    user1.email = String::from("anotheremail@example.com");
    println!("user1.email is now: {}", user1.email);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
