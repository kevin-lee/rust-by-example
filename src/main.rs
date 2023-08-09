mod hello;
mod primitives;
mod string_ops;

mod custom_types {
  pub mod structs;

  pub mod enums {
    pub mod enums;
    pub mod use_enums;
    pub mod c_like;
    pub mod linked_list;
  }

  pub mod constants;
}

mod variable_bindings;

use custom_types::constants;
use custom_types::enums;
use custom_types::structs;

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

  println!("\n===== enum - linked list (Cons list) =====");
  use enums::linked_list::*;

  let nil = List::<i32>::empty();
  println!("              Nil: {:?}", nil);

  let i32_nums: List<i32> = list!(1, 2, 3, 4, 5);
  println!("        List<i32>: {:?}", i32_nums);
  println!("i32_nums length(): {}", i32_nums.length());
  println!("i32_nums.render(): {}", i32_nums.render());

  let mut i32_nums2 = list!(1, 2, 3);
  i32_nums2 = i32_nums2.prepend(0).prepend(-1).prepend(-2).prepend(-3);
  println!("         i32_nums2: {:?}", i32_nums2);
  println!("i32_nums2.render(): {}", i32_nums2.render());
  println!("i32_nums2.length(): {:?}", i32_nums2.length());

  println!("\n===== constants =====");
  constants::constants();

  println!("\n===== variable_bindings =====");
  variable_bindings::variable_bindings();
}
