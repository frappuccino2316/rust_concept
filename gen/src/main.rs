mod rectangle;
mod calc;

use rectangle::Rectangle;
use calc::CalcArea;

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
