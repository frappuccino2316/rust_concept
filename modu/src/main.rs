mod module_a;

fn main() {
    crate::module_a::print_hello();
    crate::module_a::module_b::print_yoroshiku();
}
