use std::convert::From;

#[derive(Debug)]
#[allow(dead_code)]
struct Number {
  value: i32,
}

impl From<i32> for Number {
  fn from(item: i32) -> Self {
    Number{value: item}
  }
}

fn main() {
  let num = Number::from(30);
  println!("my number is {:?}", num);

  let int = 5;
  let num: Number = int.into();
  println!("my number is {:?}", num);

}