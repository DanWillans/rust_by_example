pub mod from_and_into;
pub mod tryfrom_and_tryinto;
pub mod to_and_from_strings;

pub fn call_all(){
  from_and_into::from_and_into();
  tryfrom_and_tryinto::tryfrom_and_tryinto();
  to_and_from_strings::to_and_from_strings();
}