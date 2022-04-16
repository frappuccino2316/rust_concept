use person::Person;

fn main() {
    let taro = person::Person::new(String::from("taro"), 50);
    let hanako = person::Person::new(String::from("hanako"), 48);

    let sato = Parents::new(&taro, &hanako);
    println!("{:?}", sato);
}

mod person {
    #[derive(Debug)]
    pub struct Person {
        pub name: String,
        pub age: u8,
    }

    impl Person {
        pub fn new(name: String, age: u8) -> Person {
            Person { name, age }
        }
    }
}

#[derive(Debug)]
struct Parents<'a, 'b> {
    father: &'a person::Person,
    mother: &'b person::Person,
}

impl<'a, 'b> Parents<'a, 'b> {
    fn new(father: &'a person::Person, mother: &'b person::Person) -> Parents<'a, 'b> {
        Parents{father, mother}
    }
}
