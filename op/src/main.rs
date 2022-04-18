fn main() {
    let rr1 = func_ex_unwrap(5);
    println!("rr1: {}", rr1.expect("Panic 1"));

    let rr2 = func_ex_unwrap(-5);
    println!("rr1: {}", rr2.expect("Panic 2"));
}

fn func_ex_unwrap(x: i32) -> Option<i32> {
    if x >= 0 {
        Some(x)
    } else {
        None
    }
}
