use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[macro_export]
macro_rules! myvec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    let a = myvec!([1, 2, 3, 4]);
    println!("{:?}", a);

    //
    Pancakes::hello_macro();
}
