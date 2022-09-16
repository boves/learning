use crate::List::{Cons, Nil};

enum List {
    Cons(i32, List),
    Nil,
}

fn main() {
    println!("Using a Box<t>");
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Cons(2, Cons(3, Nil)));
}

