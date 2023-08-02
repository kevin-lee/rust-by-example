mod hello;
mod primitives;
mod string_ops;
mod custom_types {
    pub mod structs;
    pub mod enums;
}
use custom_types::structs;
use crate::custom_types::enums;

fn main() {
    println!("===== hello world =====");
    hello::hello_world();
    println!("\n===== primitives =====");
    primitives::primitives();
    println!("\n===== structs =====");
    structs::structs();
    println!("\n===== enum =====");
    enums::enum_validation();
}
