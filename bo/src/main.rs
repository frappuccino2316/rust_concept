fn main() {
    let boxed = Box::new(1);
    let val = *boxed;

    println!("val = {}", val);
}
