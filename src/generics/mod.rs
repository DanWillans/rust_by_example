pub mod generics;
pub mod functions;
pub mod implementation;
pub mod traits;
pub mod bounds;
pub mod multiple_bounds;
pub mod where_clauses;
pub mod new_type_idiom;
pub mod associated_items;
pub mod phantom_types;

pub fn call_all(){
  generics::generics();
  functions::functions();
  implementation::implementation();
  traits::traits();
  bounds::bounds();
  multiple_bounds::multiple_bounds();
  where_clauses::where_clauses();
  new_type_idiom::new_type_idiom();
  associated_items::associated_items();
  phantom_types::phantom_types();
}