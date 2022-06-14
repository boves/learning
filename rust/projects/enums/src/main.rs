enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        // method body
    }
}

enum Option<T> {
    None,
    Some(T)
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");
    
    let absent_number: std::option::Option<i32> = None;

}

