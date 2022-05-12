// 1. Convert temperatures between Fahrenheit and Celsius.
// 2. Generate the nth Fibonacci number.
// 3. Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.


fn main() {
   
   println!(""); 
    // TODO: collect input for scale and temp
   println!("Input is c, so...");
   convert_temp('c'); //, 20);
   println!("Input is f, so...");
   convert_temp('f'); //, 20);
}

fn convert_temp (scale: char){//, temp: f64) -> char { // &str {
   

    if scale == 'c'{
        println!(" -Scale is Celsius");
    }

     else{
         println!(" -Scale is Fahrenheit.");
     }

}
