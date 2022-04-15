fn main() {
    let ary = [0, 1, 2, 3];
    for aa in &ary {
        println!("{}", aa);
    }
    println!("{}", ary[1])
}
