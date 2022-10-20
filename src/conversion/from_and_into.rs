#[derive(Debug)]
struct Number {
    value: i32,
}

impl std::convert::From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

impl std::convert::Into<i32> for Number {
  fn into(self) -> i32 {
    self.value
  }
}

pub fn from_and_into() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);
    let int: i32 = num.into();
    println!("My number is {:?}", int);
}
