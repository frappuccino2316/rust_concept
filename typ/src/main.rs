fn main() {
    let mut s = "Hello".to_string();
    let ss = String::from("Hello!!");

    s.push_str(", World!");

    println!("{}", s);
    println!("{}", ss);
    println!("{}", &s[0..3]);
}
