mod solutions;
mod utils;
use solutions::{day01, day02, day03, day04};
use utils::log::log;

fn main() {
    day01::part_one();
    day01::part_two();
    day02::part_one();
    day02::part_two();
    day03::part_one();
    day03::part_two();
    log(&day04::part_one());
    log(&day04::part_two());
}
