const LANGUAGE: &str = "Rust";
static MAGIC_NUMBER: i32 = 999;

pub fn constants() -> () {
  println!("Language: {LANGUAGE}");
  println!("magic number: {MAGIC_NUMBER}");
}