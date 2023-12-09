#![allow(dead_code)]

enum WebEvent {
  PageLoad,
  PageUnload,
  KeyPress(char),
  Paste(String),
  Click{x:i64,y:i64}
}

enum Status {
  Rich,
  Poor,
}

fn inspect(event: WebEvent) {
  match event {
    WebEvent::PageLoad => println!("page loaded"),
    WebEvent::PageUnload => println!("page unload"),
    WebEvent::KeyPress(c) => println!("press: {}", c),
    WebEvent::Paste(c) => println!("Paste: {}",c),
    WebEvent::Click{x,y} => {
      println!("clicked at x = {}, y = {}", x,y);
    }
  }
}


fn main() {
  let pressed = WebEvent::PageLoad;
  inspect(pressed);

  use Status::{Poor,Rich};

  let status = Poor;
  match status {
    Poor => println!("the poor have no money"),
    Rich => println!("the rich have lots of money"),
  }

}