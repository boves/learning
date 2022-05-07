fn main() {
    let number = 7;

    if number  < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number2 = 3;

    if number2 != 0{
        println!("number2 was something other than zero");
    }

    let number3 = 6;

    if number3 % 4 == 0 {
        println!("number is divisible by 4");
    } else if number3 % 3 == 0 {
        println!("number3 is divisible by 3");
    } else if number3 % 2 == 0{
        println!("number3 is divisible by 2");
    } else {
        println!("number3 is not divisible by 4, 3, or 2");
    }
}
