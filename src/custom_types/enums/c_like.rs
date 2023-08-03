enum ZeroOneTwoThree {
  Zero,
  One,
  Two,
  Three,
}

#[derive(Debug)]
enum Rgb {
  Red = 0xff0000,
  Green = 0x00ff00,
  Blue = 0x0000ff,
}

pub fn print_0123() -> () {
  println!("ZeroOneTwoThree::Zero={}", ZeroOneTwoThree::Zero as i32);
  println!("ZeroOneTwoThree::One={}", ZeroOneTwoThree::One as i32);
  println!("ZeroOneTwoThree::Two={}", ZeroOneTwoThree::Two as i32);
  println!("ZeroOneTwoThree::Three={}", ZeroOneTwoThree::Three as i32);
}

pub fn print_rgb() -> () {
  println!("  {:?} is #{:06x}", Rgb::Red, Rgb::Red as i32);
  println!("{:?} is #{:06x}", Rgb::Green, Rgb::Green as i32);
  println!(" {:?} is #{:06x}", Rgb::Blue, Rgb::Blue as i32);
}