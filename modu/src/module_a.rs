#[path = "module_b.rs"]
pub mod module_b;

pub fn print_hello() {
    module_b::print_yoroshiku();
}
