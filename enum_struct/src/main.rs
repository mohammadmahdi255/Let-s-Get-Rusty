#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    _Quit,
    _Move {x: i32, y: i32},
    _Write(String),
    _ChangeColor(i32, i32, i32),
}

impl Message {
    fn information() {
        println!("Let's Get Rusty");
    }
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: u32,
}

#[derive(Debug)]
enum IranCitys {
    _Ardebil,
    Tehran,
    _Mashahd,
    _Tabrize,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(IranCitys),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            0
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("City Quarter from is {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn main() {

    let _six = IpAddrKind::V6(String::from("1241:32423:234@:2342:234:324"));

    let localhost = IpAddr{
        kind: IpAddrKind::V4(127,0,0,0),
        address: 0x7f000001,
    };

    println!("{:#?}", localhost);
    Message::information();

    let some_number = Some(10);
    let _some_string = Some("a string");
    let _absent_number: Option<i32> = None;

    let _sum = match some_number {
        Some(num) => num + 10,
        None => 0,
    };

    let _sum = 10 + some_number.unwrap_or(0);
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Quarter(IranCitys::Tehran));
    println!("{:?}", plus_one(Some(10)));

    let some_value = Some(0);
    match some_value {
        Some(0) => println!("{:?}", some_value),
        _ => (),
    }

    if let Some(0) = some_value {
        println!("Zero!");
    } 
}
