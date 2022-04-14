fn main() {
    let s = 1;
    myprint(s);
    myprint(s);
}

fn myprint<T: std::fmt::Display>(msg: T) {
    println!("{}", msg);
}
