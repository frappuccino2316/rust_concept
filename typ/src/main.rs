use thiserror::Error;

fn main() {
    print_mydiv(5, 2);
    print_mydiv(-5, 0);
    print_mydiv(-5, -5);
}

fn print_mydiv(x: i32, y: i32) {
    let ret = mydiv(x, y);
    if ret.is_ok() {
        println!("no error. ans = {}", ret.unwrap());
    } else {
        println!("{}", ret.err().unwrap());
    }
}

fn mydiv(x: i32, y: i32) -> Result<i32, DivError> {
    if y == 0 {
        Err(DivError::DivByZero(x))
    } else if x < 0 && y < 0 {
        Err(DivError::BothNegative(x, y))
    } else {
        Ok(x / y)
    }
}

#[derive(Error, Debug)]
enum DivError {
    #[error("{0} divided by zero")]
    DivByZero(i32),
    #[error("Both numerator {0} and denominator {1} are negative")]
    BothNegative(i32, i32),
}
