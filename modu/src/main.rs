mod module_a;
use crate::module_a::module_b;

fn main() {
    crate::module_a::print_hello();
    module_b::print_yoroshiku();
}
