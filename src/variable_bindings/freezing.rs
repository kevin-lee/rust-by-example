pub fn freezing() -> () {
  let mut some_number = 1i32;

  println!("[0] outer some_number: {}", some_number);
  {
    // outer some_number is shadowed by this inner immutable some_number
    let some_number = some_number;

    println!("[1] inner some_number: {}", some_number);

    // Compile-time error
    // cannot assign twice to immutable variable
    // some_number = 999;
  }
  some_number = 3;
  println!("[2] outer some_number: {}", some_number);
}