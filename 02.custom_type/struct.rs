
#[derive(Debug)]
struct Person {
  name: String,
  age: u8,
}

struct Unit;

struct Pair(i32,i32);

struct Point{
  x: f32,
  y: f32,
}

#[allow(dead_code)]
struct Rectangle {
  top_left: Point,
  bottom_right: Point,
}

fn main() {
  let name = String::from("Peter");
  let age = 27;
  let peter = Person{name,age};

  println!("{:?}", peter);
  println!("{}", peter.name);
  println!("{}", peter.age);

  let point: Point = Point{x:10.3,y:0.4};
  println!("point coordinates: ({}, {})", point.x,point.y);

  let bottom_right = Point{x:5.2, ..point};
  println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

  let Point {x: left_edge, y: top_edge} = point;
  let _rectangle = Rectangle{
    top_left: Point{x:left_edge,y:top_edge},
    bottom_right: bottom_right,
  };
  println!("_rectangle {}", _rectangle.top_left.x);
  println!("_rectangle {}", _rectangle.bottom_right.x);

  let _unit = Unit;
  let pair = Pair(1,2);
  println!("pair contains {} and {}", pair.0, pair.1);

  let Pair(a,b) = pair;
  println!("pair a {} b {}", a+10, b+10);



}