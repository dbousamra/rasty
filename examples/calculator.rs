extern crate rastylib;
use std::panic;
use std::thread;

use rastylib::*;

struct Calculator {}

impl Calculator {
  pub fn add(&self, a: usize, b: usize) -> usize {
    a + b
  }

  pub fn subtract(&self, a: usize, b: usize) -> usize {
    a - b
  }

  pub fn multiply(&self, a: usize, b: usize) -> usize {
    a * b
  }

  pub fn divide(&self, a: usize, b: usize) -> usize {
    a / b
  }
}

fn main() {
  let calc = Calculator;

  let add = test_group!(
    "Add",
    [
      test!("Can add 2 and 1", { assert_eq!(2 + 1, 3) }),
      test!("Can add -1 and 1", { assert_eq!(-1 + 1, 0) }),
    ]
  );

  let subtract = test_group!(
    "Subtract",
    [
      test!("Can subtract 1 from 1", { assert_eq!(1 - 1, 0) }),
      test!("Can subtract 2 from 1", { assert_eq!(1 - 2, -1) }),
      test!("Can subtract -1 from 1", { assert_eq!(1 - -1, 2) }),
    ]
  );

  let multiply = test_group!(
    "Multiply",
    [
      test!("Can multiply 1 and 1", { assert_eq!(1 * 1, 1) }),
      test!("Can multiply 2 and 1", { assert_eq!(2 * 1, 2) }),
      test!("Can multiply -1 and 1", { assert_eq!(-1 * 1, -1) }),
    ]
  );

  let divide = test_group!(
    "Divide",
    [
      test!("Can divide 1 by 1", { assert_eq!(1 / 1, 1) }),
      test!("Can divide 2 by 1", { assert_eq!(2 / 1, 2) }),
      test!("Can divide -1 by 1", { assert_eq!(-1 / 1, -1) }),
    ]
  );

  let suite = test_group!("Calculator", [add, subtract, multiply, divide]);
  let test_runner = runners::TestRunner::new();
  test_runner.run(&suite);
}
