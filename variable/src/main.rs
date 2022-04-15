fn main() {
    let mut s = "Hello".to_string();
    println!("s={}", s);

    let s_ref = &mut s;
    let ss_ref = &mut s;
    myclear(s_ref);
    println!("s={}", s);
}

fn myclear(x: &mut String) {
    x.clear();
}