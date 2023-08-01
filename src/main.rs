mod hello;
mod primitives;
mod string_ops;
mod custom_types {
    pub mod structs;
}
use custom_types::structs;

fn main() {
    println!("===== hello world =====");
    hello::hello_world();
    println!("\n===== primitives =====");
    primitives::primitives();
    println!("\n===== structs =====");
    structs::structs();
}
