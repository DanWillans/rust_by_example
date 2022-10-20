mod formatting;
mod primitives;
mod custom_types;
mod variable_bindings;
mod types;
mod conversion;
mod expressions;

fn main() {
    println!("================= Formatting =================");
    formatting::call_all();
    println!("================= Primitives =================");
    primitives::call_all();
    println!("================= Custom Types =================");
    custom_types::call_all();
    println!("================= Variable Bindings =================");
    variable_bindings::call_all();
    println!("================= Types =================");
    types::call_all();
    println!("================= Conversion =================");
    conversion::call_all();
    println!("================= Expressions =================");
    expressions::call_all();
}
