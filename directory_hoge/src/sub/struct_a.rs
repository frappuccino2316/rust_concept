#[path = "struct_b.rs"]
pub mod struct_b;
use struct_b::StructB;

pub struct StructA {
    pub name: String,
    pub node: StructB,
}

impl StructA {
    pub fn return_name(&self) -> &String {
        &self.name
    }

    pub fn return_node_name(&self) -> &String {
        &self.node.name
    }
}
