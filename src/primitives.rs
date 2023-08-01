use crate::string_ops::ScalaLikeStringOps;

pub fn primitives() {
  println!("================\nbool\n----------------");
  let logical1: bool = true;
  println!("{logical1}");
  let logical2: bool = false;
  println!("{logical2}");

  println!("\n================\nsigned int\n----------------");
  signed_int();
  println!("\n================\nunsigned int\n----------------");
  unsigned_int();
  println!("\n================\nfloat\n----------------");
  float();

  println!("\n================\nchar\n----------------");
  char();
}

fn signed_int() {
  let i8min: i8 = 2_i16.pow(7) as i8;
  let i8max: i8 = (2_i16.pow(7) - 1) as i8;
  println!("  i8min: {i8min}");
  println!("  i8max:  {i8max}");

  let i16min: i16 = i16::MIN;
  let i16max: i16 = i16::MAX;
  println!(" i16min: {i16min}");
  println!(" i16max:  {i16max}");

  let i32min: i32 = i32::MIN;
  let i32max: i32 = i32::MAX;
  println!(" i32min: {i32min}");
  println!(" i32max:  {i32max}");

  let i64min: i64 = i64::MIN;
  let i64max: i64 = i64::MAX;
  println!(" i64min: {i64min}");
  println!(" i64max:  {i64max}");

  let i128min: i128 = i128::MIN;
  let i128max: i128 = i128::MAX;
  println!("i128min: {i128min}");
  println!("i128max:  {i128max}");
}

fn unsigned_int() {
  let u8min: u8 = 2_u16.pow(8) as u8;
  let u8max: u8 = (2_u16.pow(8) - 1) as u8;
  println!("  u8min: {u8min}");
  println!("  u8max: {u8max}");

  let u16min: u16 = u16::MIN;
  let u16max: u16 = u16::MAX;
  println!(" u16min: {u16min}");
  println!(" u16max: {u16max}");

  let u32min: u32 = u32::MIN;
  let u32max: u32 = u32::MAX;
  println!(" u32min: {u32min}");
  println!(" u32max: {u32max}");

  let u64min: u64 = u64::MIN;
  let u64max: u64 = u64::MAX;
  println!(" u64min: {u64min}");
  println!(" u64max: {u64max}");

  let u128min: u128 = u128::MIN;
  let u128max: u128 = u128::MAX;
  println!("u128min: {u128min}");
  println!("u128max: {u128max}");
}

fn float() {
  let f32min = f32::MIN;
  let f32max = f32::MAX;
  let s = format!("\
    |f32min: {f32min}
    |f32max:  {f32max}
    |");


  let f32min_max = s.strip_margin();
  println!("{f32min_max}");

  let f64min = f64::MIN;
  let f64max = f64::MAX;
  let s = format!("\
    |f64min: {f64min}
    |f64max:  {f64max}
    |");

  let f64min_max = s.strip_margin();
  println!("{f64min_max}");

}

fn char() {
  let c = 'a';
  let char_max = char::MAX;
  println!("c:{c}");
  println!("char_max:{char_max}");
}