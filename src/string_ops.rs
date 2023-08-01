pub trait ScalaLikeStringOps {
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
