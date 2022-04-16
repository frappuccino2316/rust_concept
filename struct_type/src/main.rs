fn main() {
    let mut taro = Person {
        name: String::from("taro"),
        age: 10,
    };
    taro.age = 20;

    println!("{}, {}", taro.name, taro.age);
}

struct Person {
    name: String,
    age: u8,
}
