pub mod hosting;

mod serving;

fn get_order() {
    serving::take_order();
}

