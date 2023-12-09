fn main() {
  #[derive(Debug)]
  struct Person {
    name: String,
    age: u8,
  }

  let person = Person {
    name: String::from("Alice"),
    age: 20,
  };

  // ref 表引用
  let Person{name, ref age} = person;

  println!("The person age is {}", age);
  println!("The person name is {}", name);

  // 部分引用之后，不能整体使用父级变量
  // println!("The person struct is {:?}", person);

  println!("The person age from person strcut is {:?}", person.age);

}