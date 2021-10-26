fn main() {
    let s = String::from("Hello");
    let s = takes_ownership(s);
    println!("{}", s);

    let x = 6;
    makes_copy(x);
    println!("{}", x);

    let len = calculate_length(&s);
    println!("{}", len);

    let mut s = String::from("Hello");
    change(&mut s);
    println!("{}", s);

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    s += "3";
    let r3 = &mut s;
    r3.push_str("string");
    println!("{}", r3);

    // let refrence_to_nothing = dangle();
    // println!("{}", refrence_to_nothing);

    let first = &s[..5];
    let last = &s[5..];
    println!("{}{} --- {}", first, last, s);
    let first = first_word(&s);
    // s.clear(); Error
    println!("{}", first);
}

fn takes_ownership(some_sting: String) -> String {
    println!("{}", some_sting);
    some_sting
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn calculate_length(s: &String) -> usize {
    let len = s.len();
    len
}

fn change(some_string: &mut String) {
    some_string.push_str(", World");
}

// fn dangle() -> &String {
//     let s = String::from("Hello");
//     &s
// }

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..i];
        }
    }

    s
}