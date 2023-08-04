// use vararg::vararg;

use std::fmt;

#[derive(Debug)]
pub enum List<A> {
  Nil,
  Cons { head: A, tail: Box<List<A>> },
}

/* I didn't want to use macro but I couldn't find any better way (yet). :( */
#[macro_export]
macro_rules! list {
    ($($x:expr),*) => {
       {
            let args = [$(&$x),*];
            args.iter().rfold(List::empty(), |acc, a| acc.prepend(**a))
        }
    };
}

/*
  Unfortunately, vararg doesn't seem to work. :(
  I'm getting
  have you added the `#[macro_use]` on the module/import?
  when
  ```
  list!(1, 2, 3);
  ```
  Also the function body has an issue
  ```
  args.iter().rfold(List::empty(), |acc, a| acc.prepend(a))
                                                ------- ^ expected type parameter `A`, found `&A`
                                                |
                                                arguments to this method are incorrect
  ```
  ```
  args.iter().rfold(List::empty(), |acc, a| acc.prepend(*a))
                                                        ^^ move occurs because `*a` has type `A`, which does not implement the `Copy` trait
  ```
  I don't have enough knowledge of Rust to figure it out yet.
 */
// #[vararg]
// pub fn list2<A, const L: usize>(args: [A; L]) -> List<A> {
//   args.iter().rfold(List::empty(), |acc, a| acc.prepend(*a))
// }

impl<A> List<A> {
  pub fn empty() -> List<A> {
    List::Nil
  }

  pub fn prepend(self, value: A) -> List<A> {
    List::Cons { head: value, tail: Box::new(self) }
  }

  pub fn length(&self) -> u32 {
    match self {
      List::Nil => 0,
      List::Cons { head: _, tail } => 1 + tail.length(),
    }
  }

  fn render0(&self) -> String
    where
      A: fmt::Display,
  {
    match self {
      List::Nil => "".to_string(),
      List::Cons { head, tail } => format!(", {}{}", head, tail.render0()),
    }
  }

  pub fn render(&self) -> String
    where
      A: fmt::Display,
  {
    match self {
      List::Nil => "Nil".to_string(),
      List::Cons { head, tail } => format!("List({}{})", head, tail.render0()),
    }
  }

}