fn main() {
  let immutable_box = Box::new(5u32);

  println!("immutable_box contains {}", immutable_box);

  // 可变性错误，不可变
  // *immutable_box = 4

  let mut mutable_box = immutable_box;

  println!("mutable_box contains {}", mutable_box);

  // 可变变量声明如果没有使用or修改也会有warn提示

  *mutable_box = 4;

  println!("mutable_box now contains: {}", mutable_box);

}