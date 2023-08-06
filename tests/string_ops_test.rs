use rust_by_example::string_ops::ScalaLikeStringOps;

#[test]
fn strip_margin_for_string() {
  let s = "a=123
               |b=456
               |c=789
               |";
  let expected = "a=123\nb=456\nc=789\n";

  let actual = s.strip_margin();
  assert_eq!(actual, expected)
}

#[test]
fn strip_margin_for_string_with_intentional_indent() {
  let s = "a=123
               |  b=456
               |    c=789
               |";
  let expected = "a=123\n  b=456\n    c=789\n";

  let actual = s.strip_margin();
  assert_eq!(actual, expected)
}

#[test]
fn strip_margin_for_str() {
  let s = "a=123
               |b=456
               |c=789
               |";
  let expected = "a=123\nb=456\nc=789\n";

  let actual = s.strip_margin();
  assert_eq!(actual, expected)
}

#[test]
fn strip_margin_for_str_with_intentional_indent() {
  let s = "a=123
               |  b=456
               |    c=789
               |";
  let expected = "a=123\n  b=456\n    c=789\n";

  let actual = s.strip_margin();
  assert_eq!(actual, expected)
}
