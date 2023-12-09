fn destroy_box(c:Box<i32>) {
  println!("Destroying a box that contains {}", c);
  //  c被销毁且内存释放
}

fn main() {
  let x = 5u32;

  let y = x;

  println!("x is {}, y is {}", x,y);

  // a 是一个指向堆分配的整数的指针
  let a = Box::new(5i32);
  println!("a contains: {}", a);

  // 把 a 的指针地址（非数据）复制到b 现在两者都指向
  // 同一个堆分配的数据，但是现在只有b拥有它
  // a无法访问数据，因为a不在拥有那部分堆上的内存
  let b = a;

  // 数据复制是可以的
  // let _b = a.clone();

  println!("a contains: {}", a);

  destroy_box(b);

  // 此时堆内存被释放，无法访问b
  // println!("b contains: {}",b);
}