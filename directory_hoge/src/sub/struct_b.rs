pub struct StructB {
    pub name: String,
}

impl StructB {
    pub fn print_b(&self) {
        println!("name is {}", self.name);
    }
}
