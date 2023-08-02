// An attribute to hide warnings for unused code.
#![allow(dead_code)]

use either::Either;

#[derive(Debug)]
enum NameValidationError<'a> {
  Missing,
  Invalid { value: &'a str, message: &'a str },
}

pub fn enum_validation() {
  let name1: Option<&str> = None;
  let validated_name1 = validate_name(name1);
  println!("validated_name1: {:?}", validated_name1);

  let name2: Option<&str> = Some("");
  let validated_name2 = validate_name(name2);
  println!("validated_name2: {:?}", validated_name2);

  let name3: Option<&str> = Some("1234567890123456789012345678901");
  let validated_name3 = validate_name(name3);
  println!("validated_name3: {:?}", validated_name3);

  let name4: Option<&str> = Some("Kevin");
  let validated_name4 = validate_name(name4);
  println!("validated_name4: {:?}", validated_name4);

}

fn validate_name(maybe_name: Option<&str>) -> Either<NameValidationError, &str> {
  match maybe_name {
    Option::None =>
      Either::Left(NameValidationError::Missing),

    Option::Some("") =>
      Either::Left(NameValidationError::Invalid { value: "", message: "Name should be non-empty String but an empty String is given." }),

    Option::Some(name) =>
      if name.len() > 30 {
        Either::Left(NameValidationError::Invalid { value: name, message: "Name should be less than 30 chars." })
      } else {
        Either::Right(name)
      }
  }
}