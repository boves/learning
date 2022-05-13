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

fn the_twelve_days_of_christmas() {
On the twelfth day of Christmas, my true love sent to me

verses = ["A partridge in a pear tree", "Two turtle doves, and", "Three french hens", 
    "Four calling birds", "Five golden rings", "Seven swans a-swimming", 
    "Six geese a-laying", "Eight maids a-milking",  "Nine ladies dancing", 
    "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"]




ordinals = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "twelfth"] 
opening_line = "On the {:?} day of Christmas, my true love sent to me"
    
}