pub fn for_and_range() {
  // `n` will take the values: 1, 2, ..., 100 in each iteration
  for n in 1..101 {
      if n % 15 == 0 {
          println!("fizzbuzz");
      } else if n % 3 == 0 {
          println!("fizz");
      } else if n % 5 == 0 {
          println!("buzz");
      } else {
          println!("{}", n);
      }
  }

  // A range that is inclusive of both ends using ..=
  for n in 1..=100 {
      if n % 15 == 0 {
          println!("fizzbuzz");
      } else if n % 3 == 0 {
          println!("fizz");
      } else if n % 5 == 0 {
          println!("buzz");
      } else {
          println!("{}", n);
      }
  }

  // For and Iterators
  let names = vec!["Bob", "Frank", "Ferris"];

  // .iter() borrows the collection leaving it ready for reuse.
  for name in names.iter() {
      match name {
          &"Ferris" => println!("There is a rustacean among us!"),
          // TODO ^ Try deleting the & and matching just "Ferris"
          _ => println!("Hello {}", name),
      }
  }

  println!("names: {:?}", names);

  let names = vec!["Bob", "Frank", "Ferris"];

  // .into_iter() moves the collection into loop. It cannot be used.
  for name in names.into_iter() {
      match name {
          "Ferris" => println!("There is a rustacean among us!"),
          _ => println!("Hello {}", name),
      }
  }
  
  // println!("names: {:?}", names);
  // FIXME ^ Comment out this line

  let mut names = vec!["Bob", "Frank", "Ferris"];

  // .iter_mut() mutable borrow the collection allowing for it to be modified in place.
  for name in names.iter_mut() {
      *name = match name {
          &mut "Ferris" => "There is a rustacean among us!",
          _ => "Hello",
      }
  }

  println!("names: {:?}", names);
}