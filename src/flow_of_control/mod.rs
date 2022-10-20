pub mod if_else;
pub mod loop_control;
pub mod while_control;
pub mod for_and_range;
pub mod match_control;

pub fn call_all(){
  if_else::if_else();
  loop_control::loop_control();
  while_control::while_control();
  for_and_range::for_and_range();
  match_control::match_control();
}