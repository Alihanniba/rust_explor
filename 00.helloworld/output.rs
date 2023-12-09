fn main() {
  println!("{} days", 123);

  println!("{0}, this is {1}.{1}, this is {0}", "alice", "bob");

  println!("{aa} {nn} {a22}", aa="11", nn="22", a22="33");

  println!("{} of {:b} people know binary, the other half dont",1,3);

  println!("{number:>width$}", number=1,width=6);
  println!("{number:>0width$}", number=1,width=6);

  println!("{0}  {1}","00","11");

  #[derive(Debug)]
  struct Structure(i32);


  println!("this struct {:#?} wonit print...", Structure(3));
}