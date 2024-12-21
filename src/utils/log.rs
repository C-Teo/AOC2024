use std::fmt::Debug;

pub fn log<T: Debug>(input: T) {
    println!("{:?}", input);
}
