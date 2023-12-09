fn main() {
  let mut an_integer = 1u32;
  let a_boolean = true;
  let unit = ();

  let copied_integer = an_integer;

  println!("An integer: {:?}", copied_integer);
  println!("A boolean: {:?}", a_boolean);
  println!("Meet the unit value: {:?}", unit);

  let _unused_variable = 3u32;

  an_integer = 2u32;

  println!("An integer: {:?}", an_integer);


  let _noisy_unused_variable = 2u32;
}