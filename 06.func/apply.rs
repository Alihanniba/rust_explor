fn apply<F>(f:F) where
  F: FnOnce() {
    f();
  }

fn apply_to_3<F>(f:F) -> i32 where
  F: Fn(i32) -> i32 {
    f(3)
  }

fn main() {
  use std::mem;
  let greeting = "Hello";

  let mut farewell =  "goodbyt".to_owned();

  let diary = || {
    println!("I said {}.", greeting);

    farewell.push_str("!!!");

    println!("then i screamed {}.", farewell);

    println!("noe i can sleep.zzz");
    mem::drop(farewell);
  };

  apply(diary);

  let double = |x|2*x;

  println!("3 doubled: {}", apply_to_3(double));
}