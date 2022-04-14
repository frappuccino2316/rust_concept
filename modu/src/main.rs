mod module_a;
use crate::module_a::module_b::print_yoroshiku;

fn main() {
    crate::module_a::print_hello();
    print_yoroshiku();
}
