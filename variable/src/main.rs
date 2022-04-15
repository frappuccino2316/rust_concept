fn main() {
    let mut x = 1;
    let x_ref = &x;

    x = 2;
    println!("{}", x_ref);
}

// fn myclear(x: &mut String) {
//     x.clear();
// }
