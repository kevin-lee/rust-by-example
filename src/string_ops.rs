pub trait ScalaLikeStringOps {
  /// Returns a string slice with leading whitespace and | removed
  /// It removes the leading whitespace chars then remove the subsequent '|' if found.
  /// # Examples
  /// ## For `String`,
  ///
  /// ```
  /// use rust_by_example::string_ops::ScalaLikeStringOps;
  ///
  /// let actual = format!("a=123
  ///                      |b=456
  ///                      |c=789
  ///                      |").strip_margin();
  /// let expected = "a=123
  /// b=456
  /// c=789
  /// ";
  ///
  /// assert_eq!(actual, expected);
  /// ```
  /// ***
  ///
  /// ## For `str`,
  /// ```
  /// use rust_by_example::string_ops::ScalaLikeStringOps;
  ///
  /// let actual = "a=123
  ///              |b=456
  ///              |c=789
  ///              |".strip_margin();
  ///
  /// let expected = "a=123
  /// b=456
  /// c=789
  /// ";
  /// assert_eq!(actual, expected);
  /// ```
  /// ***
  ///
  /// ## Intentional indentation with `|`
  ///
  /// ```
  /// use rust_by_example::string_ops::ScalaLikeStringOps;
  ///
  /// let actual = "a=123
  ///              |  b=456
  ///              |    c=789
  ///              |".strip_margin();
  ///
  /// let expected = "a=123
  ///   b=456
  ///     c=789
  /// ";
  /// assert_eq!(actual, expected);
  /// ```
  fn strip_margin(&self) -> String;
}

impl<A: AsRef<str>> ScalaLikeStringOps for A {
  fn strip_margin(&self) -> String {
    self.as_ref()
      .lines()
      .map(|line| line.trim_start().trim_start_matches('|'))
      .collect::<Vec<&str>>()
      .join("\n")
  }
}


#[cfg(test)]
mod tests {
  use super::*;


  #[test]
  fn strip_margin_for_string() {
    let s: String = format!("a=123
                            |b=456
                            |c=789
                            |");
    let expected = "a=123\nb=456\nc=789\n";

    let actual = s.strip_margin();
    assert_eq!(actual, expected)
  }

  #[test]
  fn strip_margin_for_string_with_intentional_indent() {
    let s: String = format!("a=123
                            |  b=456
                            |    c=789
                            |");
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

}
