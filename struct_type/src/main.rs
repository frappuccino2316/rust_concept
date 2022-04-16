fn main() {
    // let mut taro = person::Person::new(String::from("taro"), 10);

    // println!("{}", taro.name);

    // let age_plus1 = taro.age_incr(1);
    // println!("{}", age_plus1);

    // taro.age_incr_replace(10);
    // println!("{}", taro.age);

    // println!("{:?}", taro);

    let taro = person::Person::new(String::from("taro"), 50);
    let hanako = person::Person::new(String::from("hanako"), 48);

    let sato = Parents {
        father: taro,
        mother: hanako,
    };
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

        // pub fn age_incr(&self, incr: u8) -> u8 {
        //     self.age + incr
        // }

        // pub fn age_incr_replace(&mut self, incr: u8) {
        //     self.age += incr;
        // }
    }
}

#[derive(Debug)]
struct Parents {
    father: person::Person,
    mother: person::Person,
}
