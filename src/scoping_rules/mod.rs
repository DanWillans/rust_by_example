pub mod raii;
pub mod ownership_and_moves;
pub mod borrowing;
pub mod mutable_borrowing;
pub mod aliasing;
pub mod the_ref_pattern;
pub mod lifetimes;

pub fn call_all(){
  raii::raii();
  ownership_and_moves::ownership_and_moves();
  borrowing::borrowing();
  mutable_borrowing::mutable_borrowing();
  aliasing::aliasing();
  the_ref_pattern::the_ref_pattern();
  lifetimes::call_all();
}