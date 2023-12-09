fn main() {
  fn function (i:i32) -> i32 {i+1}
  let closuer_annotated = |i:i32| -> i32{i+1};
  let closure_inferred = |i| i+1;

  let i = 1;
  println!("function: {}", function(1));
  println!("closuer_annotated: {}", closuer_annotated(i));
  println!("closure_inferred: {}", closure_inferred(i));

  let one = || 1;
  println!("closure returning one: {}", one());

  let haystack = vec![1,2,3];

  let contains = move |needle| haystack.contains(needle);

  println!("{}", contains(&1));
  println!("{}", contains(&4));
  println!("{}", contains(&1));

}