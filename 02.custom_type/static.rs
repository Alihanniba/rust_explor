static LAN: &'static str = "Rust";
static THR: i32 = 10;

fn is_big(n:i32) -> bool {
  n > THR
}


fn main() {
  let n = 16;

  println!("This is {}", LAN);
  println!("The threshold is {}", THR);
  println!("{} is {}", n, if is_big(n) {"big"} else {"small"});

  // THR = 5;
}