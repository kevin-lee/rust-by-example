pub fn scope() -> () {
  let long_lived_binding = 999;

  {
    let short_lived_binding = 0;
    println!("Short lived: {}", short_lived_binding);
  }

  println!("Long lived: {}", long_lived_binding);

  // Compile-time error. short_lived_binding doesn't exist here.
  // println!("Short lived: {}", short_lived_binding);
}

pub fn shadowing() -> () {
  let variable_for_shadowing = 0;

  println!("[0] outer before shadowing: {}", variable_for_shadowing);

  {
    println!("[1] inner before shadowing: {}", variable_for_shadowing);

    let variable_for_shadowing = 1;

    println!("[2] inner after shadowing: {}", variable_for_shadowing);
  }
  println!("[3] outer before outer shadowing: {}", variable_for_shadowing);

  let variable_for_shadowing = 2;

  println!("[4] outer after outer shadowing: {}", variable_for_shadowing);
}