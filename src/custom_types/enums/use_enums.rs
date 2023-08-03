#[derive(Debug)]
enum Status {
  Active,
  Inactive,
}

pub fn use_enums() -> () {
  use Status::{Active, Inactive};

  let active = Active;
  let inactive = Inactive;

  check_status(active);
  check_status(inactive);
}

fn check_status(status: Status) -> () {
  use Status::{Active, Inactive};
  match status {
    Active => println!("Status: {:?}", Active),
    Inactive => println!("Status: {:?}", Inactive),
  }
}