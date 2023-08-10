pub fn mutability() -> () {
  let _immutable_binding = 1;
  let mut mutable_binding = 1;

  println!("Before mutation: {}", mutable_binding);

  // Ok
  mutable_binding += 1;

  println!("After mutation: {}", mutable_binding);

  // Compile-time error as _immutable_binding is immutable
  // _immutable_binding += 1;
}