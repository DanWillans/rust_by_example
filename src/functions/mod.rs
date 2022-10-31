pub mod functions;
pub mod associated_functions;
pub mod closures;
pub mod higher_order_functions;
pub mod diverging_functions;

pub fn call_all(){
  functions::functions();
  associated_functions::associated_functions();
  closures::closures();
  higher_order_functions::higher_order_functions();
  diverging_functions::diverging_functions();
}