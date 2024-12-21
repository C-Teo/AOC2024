mod solutions;
mod utils;
use solutions::{day01, day02};
use utils::log::log;

fn main() {
    day01::part_one();
    day01::part_two();
    log(&day02::part_one());
    log(&day02::part_two());
}
