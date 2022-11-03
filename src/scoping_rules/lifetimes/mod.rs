pub mod lifetimes;
pub mod explicit_annotation;
pub mod functions;
pub mod methods;
pub mod structs;
pub mod traits;
pub mod bounds;
pub mod coercion;
pub mod statics;
pub mod elision;

pub fn call_all(){
  lifetimes::lifetimes();
  explicit_annotation::explicit_annotation();
  functions::functions();
  methods::methods();
  structs::structs();
  bounds::bounds();
  coercion::coercion();
  traits::traits();
  statics::statics();
  elision::elision();
}