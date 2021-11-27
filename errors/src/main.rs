use std::fs::{self, File};
use std::io::{Error, ErrorKind, Read, Write};

fn main() {
    a();

    // let _f = File::open("hello.txt").expect("Failed to open file");

    // let mut f = match File::open("hello.txt") {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(error) => panic!("Error creating file: {:?}", error),
    //         },
    //         other => panic!("Problem opening file: {:?}", other),
    //     },
    // };

    let mut f = File::create("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Error creating file: {:?}", error);
            })
        } else {
            panic!("Problem opening file: {:?}", error);
        }
    });

    let buf: &[u8] = "hello_world".as_ref();
    // match write!(f, "hello_world") {
    //     Ok(_) => println!("write hello_world is successful"),
    //     Err(err) => panic!("Error in writing file: {:?}", err),
    // };
    match f.write(buf) {
        Ok(_) => println!("write hello_world is successful"),
        Err(err) => panic!("Error in writing file: {:?}", err),
    };

    read_username_from_file().unwrap();
}

fn read_username_from_file() -> Result<String, Error> {
    // fs::read_to_string("hello.txt")
    let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    let mut f = File::open("hello.txt");

    let mut f = match f {
        Ok(f) => f,
        Err(err) => panic!("Error in opening file: {:?}", err),
    };

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(err) => Err(err),
    // }

    f.read_to_string(&mut s)?;
    Ok(s)
}

fn a() {
    b();
}

fn b() {
    c(21);
}

fn c(num: i32) {
    if num == 22 {
        panic!("Don't pass in 22!");
    }
}