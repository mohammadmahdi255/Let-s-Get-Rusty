use std::fmt::Display;
struct ImportantExcept<'a> { 
    _part: &'a str,
}

impl<'a> ImportantExcept<'a> {
    fn return_part(&self, announcement: &str) -> &'a str {
        println!("Attention please: {}", announcement);
        self._part
    }
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str, 
    ann: T, 
) -> &'a str
where T: Display,
{ 
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else { 
        y
    }
}

fn main() {
    /*
    let r;                  // ---------+-- 'a
                            //          |
    {                       //          |
        let x = 5;          // -+-- 'b  |
        r = &x;             //  |       |
    }                       // -+       |
                            //          |
    println!("r: {}", r);   //          |
                            // ---------+
    */
    
    /*
    let x = 5;              // ----------+-- 'b
                            //           |
    let r = &x;             // --+-- 'a  |
                            //   |       |
    println!("r: {}", r);   //   |       |
                            // ----------+
    */
    
    let string1 = String::from("abcd");
    let result;
    let s: &'static str = "I have static lifetime";
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("result: {}", result);

    let novel = String::from("Call me Ishmeal. Some years ago ...");
    let first_sentence = novel.split('.').next().expect("Could not find");

    let _i = ImportantExcept {
        _part: first_sentence,
    };

    println!("first_sentence: {}, {}", first_word(first_sentence), b' ');

}

fn longest<'a>(x: &'a str, y: & str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        // y
        x
    }
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