#![allow(dead_code)]

// Struct with generic types
struct Point<T> {
    x: T,
    y: T,
}

// Implementation of Point for generic types
// Requires impl<T> to indicate to the compiler that
// the the type being implemented is generic
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Implementation of Point for only f32
// Lack of <T> after impl indicates that
// the following definition is specific.
impl Point<f32> {

    // This function isn't available for
    // all data types, only f32
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    let p = Point { x: 5.0, y: 10.0};
    let dis = p.distance_from_origin();

    println!("Dis is: {:?}", dis);
}