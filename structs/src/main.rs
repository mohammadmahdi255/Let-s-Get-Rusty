struct User {
    username: String,
    _email: String,
    _sign_in_count: u64,
    _active: bool, 
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area1(&self) -> u32 {
        self.width * self.height
    }

    fn area2(self: &Rectangle) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut user1 = User {
        _email: String::from("nemati.mahdi1020@gmail.com"),
        username: String::from("mohammdmahdi"),
        _sign_in_count: 1024,
        _active: true,
    };

    let name = user1.username;
    println!("{}", name);
    user1.username = String::from("wallace123");

    let mut _user2 = build_user(String::from("email"), String::from("mahdi"));

    let mut _user3 = User {
        _email: String::from("nemati.mahdi1020@gmail.com"),
        username: String::from("mohammdmahdi"),
        .._user2
    };

    struct _Color(u8, u8, u8);
    struct _Point(i8, i8, i8);

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle::square(5);

    println!("rect {:#?}", rect);

    println!("Area {}", rect.area1());
    println!("Area {}", rect2.area2());
    println!("Area {}", rect.can_hold(&rect2));
}

fn build_user(_email: String, username: String) -> User {
    User {
        _email,
        username,
        _active: true,
        _sign_in_count: 0,
    }
}