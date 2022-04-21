fn main() {
    let x1 = GenEx { value: 1 };
    let x2 = GenEx {
        value: String::from("Hello"),
    };
    let x3: GenEx<f64> = GenEx { value: 3.0 };

    println!("x1: {}", x1.return_value());
    println!("x2: {}", x2.return_value());
    println!("x3: {}", x3.return_value());
}

struct GenEx<T> {
    value: T,
}

impl<T> GenEx<T> {
    fn return_value(self) -> T {
        self.value
    }
}
