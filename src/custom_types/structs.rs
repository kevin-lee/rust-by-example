// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
  id: i32,
  first_name: String,
  last_name: String,
}

struct Unit;

struct XYCoordinates(i32, i32);

pub fn structs() {
  let kevin = Person { id: 1, first_name: String::from("Kevin"), last_name: String::from("Lee") };
  println!("{:?}", kevin);

  let _unit = Unit;

  let xy = XYCoordinates(10, 95);
  println!("xy: ({:?}, {:?})", xy.0, xy.1);

  let XYCoordinates(x, y) = xy;
  println!("x={:?}, y={:?}", x, y)
}