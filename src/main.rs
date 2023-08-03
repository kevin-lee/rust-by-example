mod hello;
mod primitives;
mod string_ops;
mod custom_types {
    pub mod structs;
    pub mod enums {
      pub mod enums;
      pub mod use_enums;
    }
}
use custom_types::structs;
use custom_types::enums;

fn main() {
    println!("===== hello world =====");
    hello::hello_world();
    println!("\n===== primitives =====");
    primitives::primitives();
    println!("\n===== structs =====");
    structs::structs();
    println!("\n===== enum =====");
    enums::enums::enum_validation();
    enums::use_enums::use_enums();
}
