#[macro_export]
macro_rules! vec {
    ( $( $x:exr ), * ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        } 
    };
}

// URL: https://doc.rust-lang.org/book/ch19-06-macros.html
use proc_macro;
#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {

}

pub trait HelloMacro {
    fn hello_macro();
}