mod my_mod {
  fn private_function() {
    println!("called my_mod::private_function()");
  }

  pub fn function() {
    println!("called my_mod::function()");
  }

  pub fn indirect_access() {
    println!("called my_mod::indirect_access()");
    private_function()
  }

  pub mod nested{
    pub fn function() {
      println!("called my_mod::nested::function()");
    }

    pub(in crate::my_mod) fn public_function_in_my_mod() {
      println!("called my_mod::nested::public_function_in_my_mod()");
      public_function_in_nested()
    }

    pub(self) fn public_function_in_nested() {
      println!("called my_mod::nested::public_function_in_nested()");
    }

    pub(super) fn public_function_in_super_mod() {
      println!("called my_mod::nested::public_function_in_super_mod()");
    }
  }


  pub fn call_public_function_in_my_mod() {
    println!("called my_mod::call_public_function_in_my_mod()");
    nested::public_function_in_my_mod();
    println!("> ");
    nested::public_function_in_super_mod();
    
  }

  pub(crate) fn public_function_in_crate() {
    println!("called my_mod::public_function_in_crate()");
  }

  mod private_nested{
    #[allow(dead_code)]
    pub fn function() {
      println!("called my_mod::private_nested::function()");
    }
  }

}

fn function() {
  println!("called function()");
}

fn main() {
  function();

  my_mod::function();
  
  my_mod::indirect_access();
  my_mod::nested::function();
  my_mod::call_public_function_in_my_mod();

  my_mod::public_function_in_crate();

}





