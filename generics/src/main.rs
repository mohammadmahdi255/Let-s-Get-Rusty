struct Point<T, U> {
    pub x: T,
    y: U,
}

impl<T: Copy, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point{
            x: *self.x(),
            y: other.y,
        }
    }
}

impl<U> Point<U, f64> {
    fn _y(&self) -> &f64 {
        &self.y
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let largest = get_largest(number_list);

    println!("the largest number is {}", largest);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let largest = get_largest(char_list);

    println!("the largest number is {}", largest);

    let p1 = Point{x:10 ,y:120};
    let p2 = Point{x:"hello", y:'R'};
    // let p3 = Point{x:5, y:10.0};
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, P3.y = {}", p3.x, p3.y);
}

fn get_largest<T/*: PartialOrd + Copy*/>(number_list: Vec<T>) -> T 
    where T: PartialOrd + Copy,
{
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

// fn get_largest(number_list: Vec<i32>) -> i32 {
//     let mut largest = number_list[0];
//     for number in number_list {
//         if number > largest {
//             largest = number;
//         }
//     }
//     largest
// }

// fn get_largest(number_list: Vec<char>) -> char {
//     let mut largest = number_list[0];
//     for number in number_list {
//         if number > largest {
//             largest = number;
//         }
//     }
//     largest
// }