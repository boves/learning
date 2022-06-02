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

    println!("user1.username is: {}", user1.username);

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    println!("user2.username is: {}", user2.username);
    println!("user2.email is: {}", user2.email);
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // println!("color is: {:?}, and origin is {:?}.", black, origin);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
