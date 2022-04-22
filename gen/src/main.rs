// mod calc;
// mod rectangle;
// mod right_triangle;

// use calc::CalcArea;
// use rectangle::Rectangle;
// use right_triangle::RightTriangle;
mod shape;

use shape::calc::CalcArea;
use shape::rectangle::Rectangle;
use shape::right_triangle::RightTriangle;

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

fn area<T>(x: &T) -> f64
where
    T: CalcArea,
{
    x.calc_area()
}
