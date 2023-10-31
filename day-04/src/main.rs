fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    let x = 5;
    let y = x;

    println!("{}", x);
    println!("{}", y);

    let s1 = String::from("hello");
    println!("{}", s1);
    let s2 = s1;

    println!("{}", s2);

    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    println!("{}", s);
    change(&mut s);
    println!("{}", s);

    let mut s = String::from("hello");

    println!("{}", s);
    {
        let r1 = &mut s;
        r1.push_str(" hello")
    }

    println!("{}", s);

    let fw = first_word(&s);
    println!("fw {}", fw);

    let mut s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{s} {hello} {world}");

    s.push_str("another_word");

    // println!("{}", two_string(String::from("hello"), String::from("world")));
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// fn two_string(s: &str, s2: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }
