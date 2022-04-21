mod calc;
mod rectangle;
mod right_triangle;

use calc::CalcArea;
use rectangle::Rectangle;
use right_triangle::RightTriangle;

fn main() {
    let rect = Rectangle {
        width: 1.0,
        height: 2.0,
    };
    println!("{}", area(&rect));

    let right = RightTriangle {
        width: 3.0,
        height: 8.0,
    };
    println!("{}", area(&right));
}

fn area<T: CalcArea>(x: &T) -> f64 {
    x.calc_area()
}
