fn main() {
    let mut s = "Hello".to_string();
    println!("{}", s);

    let s_ref = &mut s;
    myclear(s_ref);
    println!("{}", s);

    let s_ref2 = &mut s;
    myclear(s_ref2);
    println!("{}", s);
}

fn myclear(x: &mut String) {
    x.clear();
}
