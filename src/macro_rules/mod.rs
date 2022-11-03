pub mod macro_rules;
pub mod syntax;
pub mod dry;
pub mod dsl;
pub mod variadic;

pub fn call_all(){
   macro_rules::macro_rules();
   syntax::syntax();
  //  dry::dry();
  dsl::dsl();
  variadic::variadic();
}