fn main() {
    let s = return_hello();
    println!("{}", s);
}

fn return_hello() -> &String {
    let s = "Hello".to_strong();
    &s
}