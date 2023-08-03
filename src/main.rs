mod hello;
mod primitives;
mod string_ops;

mod custom_types {
  pub mod structs;

  pub mod enums {
    pub mod enums;
    pub mod use_enums;
    pub mod c_like;
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
  println!("===== enum - validation =====");
  enums::enums::enum_validation();

  println!("\n===== enum - use =====");
  enums::use_enums::use_enums();

  println!("\n===== enum - c like =====");
  enums::c_like::print_0123();
  enums::c_like::print_rgb();
}
