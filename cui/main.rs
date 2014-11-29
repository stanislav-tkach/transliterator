extern crate core;

use std::os;

fn main()
{
  let args = os::args();

  println!("Hello, CUI world!");
  core::test();
}
