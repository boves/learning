// 1. Convert temperatures between Fahrenheit and Celsius.
// 2. Generate the nth Fibonacci number.
// 3. Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

fn main() {

    println!("");

    println!("CHAPTER 3 HOMEWORK"); 
    println!("1. Convert temperatures between F and C"); 
    convert_temp('c', 20); //, 20);
    convert_temp('f', 212);
    
    println!("*********************");
    println!("2. Generate the nth Fibonacci number.");
    let n = 10;
    let result = f(n);

    println!("The Fibonacci for n = {:?} is {:?}\n", n, result);
    println!("*********************");

    println!("3. Print the lyrics to the Christmas carol \"The Twelve Days of Christmas,\" taking advantage of the repetition in the song.");
    println!("THE TWELVE DAYS OF CHRISTMAS");

    // the_twelve_days_of_christmas();
    for day in 1..13 {
        day_intro(day);
        for gift_day in (1..(day + 1)).rev(){
            gift(
                gift_day,
                if gift_day == 1 && day != 1 {
                    "and "
                } else {
                    ""
                },
            );
        }
    }
}

fn convert_temp (scale: char, temp: i32) {// -> char { // &str {
   

    if scale == 'c'{
        println!(" Scale is Celsius");
        // println!("The temp is: {}°C", temp);
        let temperature = (temp * 9/5) + 32;
        println!("  -{}°C is: {}°F\n", temp,  temperature);
    }

     else{
         println!(" Scale is Fahrenheit.");
         let temperature = (temp - 32) * 5/9;
         println!("  -{}°F is: {}°C\n", temp, temperature); 
     }

}

fn f(n: i32) -> i32 {
    if n < 2{
        return n;
    } else {
        return f(n-1) + f(n-2);
    }
}

fn day_intro(n: u32) {
    let day = match n {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "",
    };

    println!("\nOn the {} day of Christmas\nmy true love sent me to me:", day);
}

fn gift(n: u32, prefix: &str) {
    let gift_text = match n{
        1 => "a Partridge in a Pear Tree",
        2 => "Two Turtle Doves",
        3 => "Three French Hens",
        4 => "Four Calling Birds",
        5 => "Five Golden Rings",
        6 => "Six Geese a Laying",
        7 => "Seven Swans a Swimming",
        8 => "Eight Maids a Milking",
        9 => "Nine Ladies Dancing",
        10 => "Ten Lords a Leaping",
        11 => "Eleven Pipers Piping",
        12 => "12 Drummers Drumming",
        _ => "",
    };
    println!("{}{}", prefix, gift_text);
}