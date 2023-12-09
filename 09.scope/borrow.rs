fn eat_box_i32(boxed_i32: Box<i32>) {
  println!("Destroying box that contains {}", boxed_i32);
}

fn borrow_i32(borrowed_i32: &i32) {
  println!("This int is: {}", borrowed_i32);
}

fn main() {
  let boxed_i32 = Box::new(5_i32);

  let stacked_i32 = 6_i32;

  borrow_i32(&boxed_i32);
  borrow_i32(&stacked_i32);

  {
    // 取得一个引用
    let _ref_to_i32: &i32 = &boxed_i32;

    // 如果 boxed_i32 在后面被引用。就不能在这里被借用，因为这里会销毁。
    // eat_box_i32(boxed_i32);

    borrow_i32(_ref_to_i32);

  }
  eat_box_i32(boxed_i32);
  // boxed_i32 被销毁，后面不存在对它的引用了

}