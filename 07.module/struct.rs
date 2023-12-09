mod my {
  pub struct OpenBox<T> {
    pub contents: T,
  }

  #[allow(dead_code)]
  pub struct ClosedBox<T> {
    contents: T,
  }

  impl<T> ClosedBox<T> {
    pub fn new(contents: T) -> ClosedBox<T> {
      ClosedBox {
        contents: contents,
      }
    }
  }
}

fn main() {
  let open_box = my::OpenBox{contents:"public information"};
  println!("the open box contains: {}", open_box.contents);

  let _close_box = my::ClosedBox::new("classified information");
  

}



