mod formatting;
mod primitives;
mod custom_types;
mod variable_bindings;

fn main() {
    println!("================= Formatting =================");
    formatting::call_all();
    println!("================= Primitives =================");
    primitives::call_all();
    println!("================= Custom Types =================");
    custom_types::call_all();
    println!("================= Variable Bindings =================");
    variable_bindings::call_all();
}
