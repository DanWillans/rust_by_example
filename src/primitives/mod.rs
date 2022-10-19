pub mod primitives;
pub mod literals_and_operators;
pub mod tuples;
pub mod arrays_and_slices;

pub fn call_all(){
  primitives::primitives();
  literals_and_operators::literals_and_operators();
  tuples::tuples();
  arrays_and_slices::arrays_and_slices();
}