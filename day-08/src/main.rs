use std::collections::HashMap;

fn main() {
    // Vector
    let v: Vec<i32> = Vec::new();
    println!("{:#?}", v);

    let mut v = vec![1, 2, 3];
    println!("{:?}", v);

    v.push(4);
    println!("{:?}", v);

    let mut v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    v.push(4);
    let second: &i32 = &v[2];
    println!("The third element is {second}");

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    println!("{:?}", v);

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{i}")
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);

    ////////////////////////// String ////////////////////
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    println!("{s}");

    let hello = String::from("안녕하세요");
    println!("{hello}");

    let mut s1 = String::from("foo");
    let mut s2 = String::from("bar");
    let s3 = "foo";
    s1.push_str(&s2);
    s2.push_str(s3);
    s1.push_str(s3);
    println!("s3 is {s3}");
    println!("s2 is {s2}");
    println!("s1 is {s1}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2;
    println!("s1 is {s1}");
    println!("s2 is {s2}");
    println!("s3 is {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    let hello = "안녕하세요";
    let answer = hello.bytes().nth(0).unwrap();
    let answer = hello.chars().nth(0).unwrap();
    println!("{answer}");

    for b in "안녕하세요".bytes() {
        println!("{b}");
    }

    for b in "안녕하세요".chars() {
        println!("{b}");
    }

    ////////////////////////// Hash map ////////////////////
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{:?}", score);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let mut scores = HashMap::new();
    let field_name = "Favorite color";
    let field_value = String::from("Blue");
    scores.insert(field_name, &field_value);

    println!("{:?}", scores);
    println!("{} {}", field_name, field_value);

    let mut value =  &mut "field_value".to_string();
    scores.insert(field_name, value);
    println!("{:?}", scores);
    println!("{:?}", field_name);
    println!("{:?}", value);

    // scores.insert(field_name, &"field_value_2".to_string());
    // println!("{:?}", scores);
    
    // scores.entry(field_name).or_insert(&"field_value_2".to_string());
    // println!("{:?}", scores);
    //

    // value.push_str("__2__");
    // println!("{:?}", scores);
}
