mod calc;
mod rectangle;

use calc::CalcArea;
use rectangle::Rectangle;

fn main() {
    let rect = Rectangle {
        width: 1.0,
        height: 2.0,
    };
    println!("{}", area(&rect));
}

fn area<T: CalcArea>(x: &T) -> f64 {
    x.calc_area()
}
