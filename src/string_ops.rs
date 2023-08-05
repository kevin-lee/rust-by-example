pub trait ScalaLikeStringOps {
  /// Returns a string slice with leading whitespace and | removed
  /// It removes the leading whitespace chars then remove the subsequent '|' if found.
  /// # Examples
  /// ## For `String`,
  /// ```
  /// let s = format!("a=123
  ///                 |b=456
  ///                 |c=789
  ///                 |");
  /// s.strip_margin();
  /// ```
  /// returns
  /// ```
  /// "a=123
  /// b=456
  /// c=789
  /// "
  /// ```
  /// ***
  ///
  /// ## For `str`,
  /// ```
  /// let s = "a=123
  ///         |b=456
  ///         |c=789
  ///         |".strip_margin();
  /// ```
  /// returns
  /// ```
  /// "a=123
  /// b=456
  /// c=789
  /// "
  /// ```
  /// ***
  ///
  /// ## Intentional indentation with `|`
  ///
  /// ```
  /// let s = "a=123
  ///         |  b=456
  ///         |    c=789
  ///         |".strip_margin();
  /// ```
  /// returns
  /// ```
  /// "a=123
  ///   b=456
  ///     c=789
  /// "
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
