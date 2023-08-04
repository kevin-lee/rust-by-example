pub trait ScalaLikeStringOps {
  /// Returns a string slice with leading whitespace and | removed
  /// It removes the leading whitespace chars then remove the subsequent '|' if found.
  /// # Examples
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
  fn strip_margin(&self) -> String;
}

impl ScalaLikeStringOps for String {
  fn strip_margin(&self) -> String {
    self.lines()
      .map(|line| line.trim_start().trim_start_matches('|'))
      .collect::<Vec<&str>>()
      .join("\n")
  }
}
