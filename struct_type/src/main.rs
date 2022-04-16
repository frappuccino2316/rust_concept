fn main() {
    let name = String::from("hana");
    let age = 30;

    let hana = Person { name, age };

    let jiro = Person{
        name: String::from("jiro"),
        ..hana
    };

    println!("{}, {}", hana.name, hana.age);
    println!("{}, {}", jiro.name, jiro.age);
}

struct Person {
    name: String,
    age: u8,
}
