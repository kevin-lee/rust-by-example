#[derive(Debug)]
enum Status {
  Active,
  Inactive,
}

enum Week {
  Monday,
  Tuesday,
  Wednesday,
  Thursday,
  Friday,
  Saturday,
  Sunday,
}

pub fn use_enums() -> () {
  use Status::{Active, Inactive};

  let active = Active;
  let inactive = Inactive;

  check_status(active);
  check_status(inactive);

  say_week_day(Week::Monday);
  // say_week_day(Monday); // compile-time error: not found in this scope
  {
    use Week::*;
    say_week_day(Tuesday);
    say_week_day(Wednesday);
    say_week_day(Thursday);
    say_week_day(Friday);
    say_week_day(Saturday);
    say_week_day(Sunday);
  }
}

fn check_status(status: Status) -> () {
  use Status::{Active, Inactive};
  match status {
    Active => println!("Status: {:?}", Active),
    Inactive => println!("Status: {:?}", Inactive),
  }
}

fn say_week_day(week: Week) -> () {
  use Week::*;

  let day = match week {
    Monday => "Monday",
    Tuesday => "Tuesday",
    Wednesday => "Wednesday",
    Thursday => "Thursday",
    Friday => "Friday",
    Saturday => "Saturday",
    Sunday => "Sunday",
  };
  println!("Today is {day}.")
}