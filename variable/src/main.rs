fn main() {
    let s = "Hello".to_string();
    let ss = s.clone();
    myprint(s);
    myprint(ss);
}

fn myprint<T: std::fmt::Display>(msg: T) {
    println!("{}", msg);
}
