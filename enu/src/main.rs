fn main() {
    let v: Vec<EnumExample> = vec![
        EnumExample::TupleTypeExample1(String::from("Hello")),
        EnumExample::TupleTypeExample2(10, true),
        EnumExample::StructTypeExample {
            name: String::from("taro"),
            age: 10,
        },
    ];

    for e in &v {
        if let EnumExample::StructTypeExample { name: n, age: a } = e {
            println!("StructTypeExample_iflet: name = {}, age = {}", n, a);
        }
    }

    for e in &v {
        match e {
            EnumExample::TupleTypeExample1(s) => {
                println!("TupleTypeExample1: s = {}", s);
            }
            EnumExample::TupleTypeExample2(i, b) => {
                println!("TupleTypeExample2: i = {}, b = {}", i, b);
            }
            EnumExample::StructTypeExample { name: n, .. } => {
                println!("StructTypeExample: name = {}", n);
            }
        }
    }
}

enum EnumExample {
    TupleTypeExample1(String),
    TupleTypeExample2(i32, bool),
    StructTypeExample { name: String, age: u8 },
}
