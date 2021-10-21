fn main() {
    // Integers

    let _a = 98_22_2; // Decimal
    let _b = 0xff; // Hex
    let _c = 0o77; // Octal
    let _d = 0b1111_0000; // Binary
    let _e = b'A'; // Byte

    // Floating-point numbers

    let _f = 2.0;
    let _g: f32 = 3.0;

    // addition
    let _sum = 5 + 10;
    // subtraction
    let _difference = 95.5 - 4.3;
    // multiplication
    let _product = 4 * 30;
    // division
    let _guotient = 56.7 / 32.2;
    // remainder
    let _remainder = 43 % 5;

    // Booleans

    let _t = true;
    let _y = false;

    // Character

    let _c = 'z';
    let _z = 'Z';
    let _heart_eyed_cat = 'w';

    // compound Types

    let tup = ("Let's Get Rusty!", 100_000, "OK");
    let (_channel, _sub_count, _result) = tup;
    let _channel = tup.0;
    let _sub_count = tup.1;
    let _result = tup.2;

    let error_codes = [200, 404, 500];
    let _not_found = error_codes[2];

    let _byte = [0; 8];

    // let x; Error

    let mut x = 5;
    println!("the value of x is: {}", x);
    x = 9;
    println!("The value of x is: {}", x);
    let x = "six";
    println!("The value of x is: {}", x);

    const SUBSCRIBER_COUNT: u32 = 1000_000;

    println!("The value of SUBSCRIBER_COUNT is: {}", SUBSCRIBER_COUNT);

    let sum = my_function(10, -20);
    println!("The value of sum is: {}", sum);

    control_flow(sum);

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    let number = match condition {
        true => 11,
        false => 9,
    };
    println!("The value of number is: {}", number);

    let mut counter = 0;
    let result = loop {
        println!("again!");
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    println!("the result is {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30 , 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in 1..10 {
        println!("{}!", number);
    }

}

fn my_function(x: i32, y: i32) -> i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    let sum = x + y;
    return sum;
    // or x + y
}

fn control_flow(number: i32) {
    if (number < 10 && number > -100) || number > 100  {
        println!("first condition was true");
    } else if number < 22 {
        println!("second condition was true");
    } else {
        println!("condition was false");
    }
}
