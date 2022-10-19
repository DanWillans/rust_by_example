pub mod formatted_print;
pub mod formatted_print_debug;
pub mod formatted_print_display;
pub mod formatted_print_formatting;

pub fn call_all(){
  formatted_print::formatted_print();
  formatted_print_debug::formatted_print_debug();
  formatted_print_display::formatted_print_display();
  formatted_print_formatting::formatted_print_formatting();
}