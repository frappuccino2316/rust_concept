use std::cmp::Ordering;

fn main() {
    print_sign(determine_sign(1));
    print_sign(determine_sign(-2));
    print_sign(determine_sign(0));
}

fn determine_sign(x: i32) -> Sign {
    match x.cmp(&0) {
        Ordering::Greater => Sign::Positive,
        Ordering::Less => Sign::Negative,
        Ordering::Equal => Sign::Zero
    }
}

fn print_sign(s: Sign) {
    match s {
        Sign::Positive => println!("+"),
        Sign::Zero => println!("0"),
        Sign::Negative => println!("-"),
    }
}

enum Sign {
    Positive,
    Zero,
    Negative,
}
