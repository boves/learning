fn main() {
    // full notation for Vec<T> (vector) collection
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3,];

    let mut v3 = Vec::new(); 

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    println!("{:?}", v3);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match.v.get(2) {
        Some(third) => println!("The third element of {}", third),
        None => println!("There is no third element."),
    }



    // do stuff with  v2
} // <- v2 goes out of scope and is freed here