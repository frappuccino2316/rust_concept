fn main() {
    let taro = Person {
        name: String::from("taro"),
        age: 10,
    };

    println!("{}, {}", taro.name, taro.age);
}

struct Person {
    name: String,
    age: u8,
}
