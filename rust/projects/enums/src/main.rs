enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(string),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        // method body
}

fn main() {
    let m = Message:Write(String::from("hello"));
    m.call();

}

