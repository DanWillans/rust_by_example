pub mod visibility;
pub mod struct_visibility;
pub mod the_use_declaration;
pub mod super_and_self;
pub mod file_hierarchy;

pub fn call_all(){
  visibility::visibility();
  struct_visibility::struct_visibility();
  the_use_declaration::the_use_declaration();
  super_and_self::super_and_self();
  file_hierarchy::file_hierarchy();
}