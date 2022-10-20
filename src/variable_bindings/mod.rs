pub mod variable_bindings;
pub mod mutability;
pub mod scope_and_shadowing;
pub mod declare_first;
pub mod freezing;

pub fn call_all(){
  variable_bindings::variable_bindings();
  mutability::mutability();
  scope_and_shadowing::scope_and_shadowing();
  declare_first::declare_first();
  freezing::freezing();
}

