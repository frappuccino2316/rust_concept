use crate::RecursiveEnum::{Val, Null};

fn main() {
    let x = Val(Box::new(Val(Box::new(Null))));
    println!("{:?}", x);
}

#[derive(Debug)]
enum RecursiveEnum {
    Val(Box<RecursiveEnum>),
    Null
}