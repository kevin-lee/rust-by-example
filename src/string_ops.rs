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
