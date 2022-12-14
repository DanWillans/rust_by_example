// impl<A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}

// // Expressing bounds with a `where` clause
// impl<A, D> MyTrait<A, D> for YourType
// where
//     A: TraitB + TraitC,
//     D: TraitE + TraitF,
// {
// }

use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

// Because we would otherwise have to express this as `T: Debug` or
// use another method of indirect approach, this requires a `where` clause:
impl<T> PrintInOption for T
where
    Option<T>: Debug,
{
    // We want `Option<T>: Debug` as our bound because that is what's
    // being printed. Doing otherwise would be using the wrong bound.
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

trait PrintInOption2 {
  fn print_in_option_2(self);
}

impl<T: Debug> PrintInOption2 for T {
  fn print_in_option_2(self) {
      println!("{:?}", Some(self));
  }
}

pub fn where_clauses() {
    let vec = vec![1, 2, 3];
    let vec_2 = vec![1, 2, 3];

    vec.print_in_option();
    vec_2.print_in_option_2();
}
