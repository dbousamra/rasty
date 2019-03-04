#![feature(fn_traits)]
#![feature(duration_float)]
use std::fmt;
use std::time::Duration;

pub mod runners {

  use colored::*;
  use crate::*;
  use std::time::Duration;
  use std::time::Instant;

  pub struct TestRunner {}

  impl TestRunner {
    pub fn new() -> TestRunner {
      TestRunner {}
    }

    pub fn run(&self, suite: &TestSuite) -> TestRunResult {
      let max_offset = self.get_max_offset(suite);
      let results = self.run_with_offset(suite, 0, max_offset);

      let mut duration = Duration::from_millis(0);
      let mut total = 0;
      let mut passed = 0;
      let mut failed = 0;

      for result in results {
        let (assert, time_taken) = result;
        duration += time_taken;
        total += 1;

        if assert.is_success() {
          passed += 1
        } else {
          failed += 1;
        };
      }

      let test_run_result = TestRunResult {
        duration,
        total,
        passed,
        failed,
      };

      println!("");

      let total_time_taken_seconds_string =
        format!("{:.*}s", 2, test_run_result.duration.as_float_secs());

      if test_run_result.failed == 0 {
        let output = format!(
          "All {} tests passed ({})",
          test_run_result.total, total_time_taken_seconds_string
        )
        .green();
        println!("{}", output)
      } else {
        let output = format!(
          "{} out of {} tests failed ({})",
          test_run_result.failed, test_run_result.total, total_time_taken_seconds_string
        )
        .red()
        .bold();
        println!("{}", output)
      }

      test_run_result
    }

    fn run_test(
      &self,
      name: &String,
      assertion: &Assertion,
      current_offset: usize,
      max_offset: usize,
    ) -> Vec<(AssertionResult, Duration)> {
      let test_name_spaces = " ".repeat(current_offset);
      let test_name_string = format!("{}{}:", test_name_spaces, name);

      let offset = (max_offset + 2) - test_name_string.chars().count();
      print!("{}", test_name_string);

      let start = Instant::now();
      let result = assertion.run();
      let elapsed = Instant::now().duration_since(start);
      let elapsed_seconds_string = format!("{:.*}s", 2, elapsed.as_float_secs());

      let test_result_spaces = " ".repeat(offset);

      match result.clone() {
        AssertionResult::Success() => {
          let output = format!("{}OK ({})\n", test_result_spaces, elapsed_seconds_string).green();
          print!("{}", output)
        }
        AssertionResult::Failure(reason) => {
          let output = format!("{}FAIL ({})\n", test_result_spaces, elapsed_seconds_string)
            .red()
            .bold();
          print!("{}", output);

          let failure_spaces = " ".repeat(current_offset + 2);
          let lines = reason.split("\n");
          for line in lines {
            let output = format!("{}{}\n", failure_spaces, line).red();
            print!("{}", output)
          }
        }
      };
      vec![(result, elapsed)]
    }

    fn run_test_group(
      &self,
      name: &String,
      group: &Vec<TestSuite>,
      current_offset: usize,
      max_offset: usize,
    ) -> Vec<(AssertionResult, Duration)> {
      println!("{}{}", " ".repeat(current_offset), name);
      group
        .iter()
        .flat_map(|test| self.run_with_offset(test, current_offset + 2, max_offset))
        .collect()
    }

    fn run_with_offset(
      &self,
      suite: &TestSuite,
      current_offset: usize,
      max_offset: usize,
    ) -> Vec<(AssertionResult, Duration)> {
      match suite {
        TestSuite::Test(name, assertion) => {
          self.run_test(name, assertion, current_offset, max_offset)
        }
        TestSuite::TestGroup(name, tests) => {
          self.run_test_group(name, tests, current_offset, max_offset)
        }
      }
    }

    pub fn get_max_offset(&self, suite: &TestSuite) -> usize {
      fn go(suite: &TestSuite, indent: usize, max: usize) -> usize {
        match suite {
          TestSuite::TestGroup(_, tests) => tests
            .iter()
            .map(|test| go(test, indent + 2, max))
            .max()
            .unwrap_or(0),
          TestSuite::Test(name, _) => indent + name.chars().count(),
        }
      };
      go(suite, 0, 0)
    }
  }
}

pub struct TestRunResult {
  duration: Duration,
  total: usize,
  passed: usize,
  failed: usize,
}

#[derive(Debug, Clone)]
pub enum AssertionResult {
  Success(),
  Failure(String),
}

impl AssertionResult {
  pub fn is_success(&self) -> bool {
    match self {
      AssertionResult::Success() => true,
      AssertionResult::Failure(_) => false,
    }
  }
}

pub struct Assertion {
  pub f: Box<Fn() -> AssertionResult>,
}

impl Assertion {
  pub fn run(&self) -> AssertionResult {
    self.f.call(())
  }
}

impl fmt::Debug for Assertion {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Assertion!")
  }
}

#[derive(Debug)]
pub enum TestSuite {
  Test(String, Assertion),
  TestGroup(String, Vec<TestSuite>),
}

#[macro_export]
macro_rules! test {
  ( $name:expr, $x:expr ) => {{
    fn inner() -> AssertionResult {
      panic::set_hook(Box::new(|_info| {
        // do nothing
      }));

      let result = match panic::catch_unwind(|| $x) {
        Ok(_) => AssertionResult::Success(),
        Err(error) => {
          use std::borrow::Cow;
          let error_as_str = error.downcast_ref::<&str>().map(|s| Cow::from(*s));
          let error_as_string = error.downcast_ref::<String>().map(|s| Cow::from(s.clone()));
          let message = error_as_str
            .or(error_as_string)
            .map(|cow| cow.to_string())
            .unwrap_or("Something terrible has happened".to_string());
          AssertionResult::Failure(message)
        }
      };
      let _ = panic::take_hook();
      result
    };

    let assertion = Assertion { f: Box::new(inner) };
    TestSuite::Test($name.to_string(), assertion)
  }};
}

#[macro_export]
macro_rules! test_group {
  ( $name:expr, [$( $test:expr $(,)*)*] ) => {{
    let mut tests_vec = Vec::new();
    $(
        tests_vec.push($test);
    )*
    TestSuite::TestGroup($name.to_string(), tests_vec)
  }};
}

fn execute_closure(closure_argument: &mut FnMut() -> ()) {
  let result = closure_argument();
}
