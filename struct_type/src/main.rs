fn main() {
    let name = String::from("hana");
    let age = 30;

    let hana = Person { name, age };

    println!("{}, {}", hana.name, hana.age);
}

struct Person {
    name: String,
    age: u8,
}
