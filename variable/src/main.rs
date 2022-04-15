fn main() {
    let v1 = [1, 2, 3, 4, 5];
    let v2 = [6, 7, 8];
    let p = pick2(&v1, &v2, 2);

    for ss in p.0 {
        println!("{}", ss);
    }
    for ss in p.1 {
        println!("{}", ss);
    }
}

fn pick2<'a, 'b>(x: &'a [i32], y: &'b [i32], end: usize) -> (&'a [i32], &'b [i32]) {
    (&x[..end], &y[..end])
}
