fn main() {
    let mut v = vec![0, 1, 2, 3];

    println!("before push: {:?}", v);
    v.push(10);
    println!("after push: {:?}", v);

    v[2] += 10;
    println!("after v[2] += 10: {:?}", v);

    println!("&v[3..] = {:?}", &v[3..]);
}
