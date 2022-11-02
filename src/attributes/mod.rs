pub mod dead_code;
pub mod crates;
pub mod cfg;

pub fn call_all(){
  dead_code::dead_code();
  crates::crates();
  cfg::cfg();
}