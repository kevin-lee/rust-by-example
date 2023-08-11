pub fn declare_first() -> () {
  let binding;

  {
    let x = 2;

    // Initialize the binding
    binding = x * x;
  }

  println!("binding in inner scope: {}", binding);

  let another_binding;

  // Compile-time error! "`another_binding` used here but it is possibly-uninitialized"
  // println!("another binding: {}", another_binding);

  another_binding = 1;

  println!("another binding with declare first: {}", another_binding);
}