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

    let v4 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v4[2];
    println!("The third element is {}", third);

    match v4.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v5 = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v5[100];
    let does_not_exist = v5.get(100);

    let mut v6 = vec![1, 2, 3, 4, 5];
    let first = &v6[0];

    v6.push(6);

    // println!("The first element is: {}", first);
    println!("The first element is: {:?}", &v6[0]);

    // iterating over the values in a Vector
    println!("\nIterating Over a Vector");
    let v = vec![100, 32, 57];
    for i in &v {
        println!("--{}", i);

    }



    // do stuff with  v2
} // <- v2 goes out of scope and is freed here