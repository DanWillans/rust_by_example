mod formatting;
mod primitives;
mod custom_types;

fn main() {
    println!("================= Formatting =================");
    formatting::call_all();
    println!("================= Primitives =================");
    primitives::call_all();
    println!("================= Custom Types =================");
    custom_types::call_all();
}
