#[macro_use]
extern crate aoc;

pub mod register {
    include!(concat!(env!("OUT_DIR"), "/register.rs"));
}
