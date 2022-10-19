// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
  // An `enum` may either be `unit-like`,
  PageLoad,
  PageUnload,
  // like tuple structs,
  KeyPress(char),
  Paste(String),
  // or c-like structures.
  Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
  match event {
      WebEvent::PageLoad => println!("page loaded"),
      WebEvent::PageUnload => println!("page unloaded"),
      // Destructure `c` from inside the `enum`.
      WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
      WebEvent::Paste(s) => println!("pasted \"{}\".", s),
      // Destructure `Click` into `x` and `y`.
      WebEvent::Click { x, y } => {
          println!("clicked at x={}, y={}.", x, y);
      },
  }
}

// Type Aliasing

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

// Use

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

// C-like

// enum with implicit discriminator (starts at 0)
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

// Testcase: Linked-list
enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

impl List {
  // Create an empty list
  fn new() -> List {
    List::Nil
  }

  // Consume a list, and return the same list with a new element at its front
  fn prepend(self, elem: u32) -> List {
    List::Cons(elem, Box::new(self))
  }

  // Return the length of the list
  fn len(&self) -> u32 {
      // `self` has to be matched, because the behavior of this method
      // depends on the variant of `self`
      // `self` has type `&List`, and `*self` has type `List`, matching on a
      // concrete type `T` is preferred over a match on a reference `&T`
      // after Rust 2018 you can use self here and tail (with no ref) below as well,
      // rust will infer &s and ref tail. 
      // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
      match *self {
          // Can't take ownership of the tail, because `self` is borrowed;
          // instead take a reference to the tail
          List::Cons(_, ref tail) => 1 + tail.len(),
          // Base Case: An empty list has zero length
          List::Nil => 0
      }
  }

  // Return representation of the list as a (heap allocated) string
  fn stringify(&self) -> String {
    match *self {
      List::Cons(head, ref tail) => {
        // `format!` is similar to `print!`, but returns a heap
        // allocated string instead of printing to the console
        format!("{}, {}", head, tail.stringify())
      },
      List::Nil => {
        format!("Nil")
      },
    }
  }
}

pub fn enums() {
  let pressed = WebEvent::KeyPress('x');
  // `to_owned()` creates an owned `String` from a string slice.
  let pasted  = WebEvent::Paste("my text".to_owned());
  let click   = WebEvent::Click { x: 20, y: 80 };
  let load    = WebEvent::PageLoad;
  let unload  = WebEvent::PageUnload;

  inspect(pressed);
  inspect(pasted);
  inspect(click);
  inspect(load);
  inspect(unload);

  // Alias
  let x = Operations::Add;
  let y = Operations::Subtract;

  // Call function with aliases
  println!("Type Alias Addition: {}", x.run(1,2));
  println!("Type Alias Subtraction: {}", y.run(1,2));

  // Use
  use Status::{Poor, Rich};
  use Work::*;

  // Equivalent to `Status::Poor`
  let status = Poor;
  // Equivalent to `Work::Civilian`
  let work = Civilian;

  // Equivalent to `Status::Poor`
  let status_2 = Rich;
  // Equivalent to `Work::Civilian`
  let work_2 = Soldier;

  match status{
    Rich => println!("The rich have lots of money!"),
    Poor => println!("The poor have no money..."),
  }

  match status_2{
    Rich => println!("The rich have lots of money!"),
    Poor => println!("The poor have no money..."),
  }

  match work{
    Civilian => println!("Civilians work!"),
    Soldier => println!("Soldiers fight!"),
  }

  match work_2{
    Civilian => println!("Civilians work!"),
    Soldier => println!("Soldiers fight!"),
  }

  // C-like
  println!("zero is {}", Number::Zero as i32);
  println!("one is {}", Number::One as i32);
  println!("two is {}", Number::Two as i32);

  println!("roses are #{:06x}", Color::Red as i32);
  println!("violets are #{:06x}", Color::Blue as i32);
  println!("bogies are #{:06x}", Color::Green as i32);


  // Linked List
  let mut list = List::new();

  // Prepend some elements
  list = list.prepend(1);
  list = list.prepend(2);
  list = list.prepend(3);

  // Show the final state of the list
  println!("linked list has length: {}", list.len());
  println!("{}", list.stringify());
}

