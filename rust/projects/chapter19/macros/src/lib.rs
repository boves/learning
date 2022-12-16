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