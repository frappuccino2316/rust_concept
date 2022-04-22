mod sub;
use sub::struct_a::struct_b::StructB;
use sub::struct_a::StructA;

fn main() {
    let b = StructB {
        name: String::from("BBB!!!"),
    };

    let a = StructA {
        name: String::from("I am AAA!!!"),
        node: b,
    };

    println!("a name is {}", a.return_name());
    println!("b name is {}", a.return_node_name());

    a.node.print_b();
}
