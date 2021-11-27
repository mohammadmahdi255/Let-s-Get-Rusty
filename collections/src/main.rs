use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;

fn main() {

    for i in (0..8).rev() {
        println!("{:?}", i);
    }

    let a = [1, 2, 3];
    let mut v:Vec<i32> = Vec::new();
    v.push(a[0]);
    v.push(a[1]);
    v.push(a[2]);

    let mut v2 = vec![1, 2, 3, 4, 5];

    let third = &mut v2[2];
    println!("The third element is {}", third);
    *third = 10;
    println!("The third element is {}", v2[2]);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element!")
    }

    for i in 0..v2.len() {
        v2[i] += 1;
    }

    for i in &mut v2 {
        *i += 5;
        println!("{}", *i);
    }

    let row = vec![
        SpreadsheetCell::Int(10),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    for i in &row {
        match i {
             SpreadsheetCell::Int(j) => println!("{}", j),
             SpreadsheetCell::Float(j) => println!("{}", j),
             SpreadsheetCell::Text(j) => println!("{}", j),
         };
    }

    let _s1 = String::new();
    let s2 = "initial contents";
    let _s3 = s2.to_string();
    let _s4 = String::from("initial contents");

    let hello = [
        String::from("Hello"),
        String::from("سلام"),
        String::from("こんにちは"),
        String::from("नमस्कार"),
        String::from("γεια σας"),
        String::from("नमस्ते"),
        ];

    for i in &hello {
        println!("{}", i);
    }

    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');

    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1.clone() + &s2;
    let s4 = format!("{}{}", s1, s2);

    println!("{} {} {} {}", s1, s2, s3, s4);

    // Bytes values
    for b in hello[5].bytes() {
        println!("{}", b);
    }

    // Scalar values
    for c in hello[5].chars() {
        println!("{}", c);
    }

    // Grapheme clusters
    for g in hello[5].graphemes(true) {
        println!("{}", g);
    }
    for g in hello[5].graphemes(false) {
        println!("{}", g);
    }

    // Hashmap

    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    scores.entry("Red".to_string()).or_insert(30);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("{}", score.unwrap_or(&0));

    for (key, value) in &scores {
        println!("{} -> {}", key, value);
    }

    let text = "hello world wonderrful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
