mod lib;

mod back;

// use lib::hello;

fn main() {
    crate::lib::hello::hello_world();
    crate::back::front::serving();
}
