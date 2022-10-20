pub mod casting;
pub mod literals;
pub mod inference;
pub mod aliasing;

pub fn call_all(){
  casting::casting();
  literals::literals();
  inference::inference();
  aliasing::aliasing();
}