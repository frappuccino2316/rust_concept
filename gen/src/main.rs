mod shape;

use shape::calc::CalcArea;
use shape::calc::CalcLength;
use shape::line::Line;
use shape::rectangle::Rectangle;
use shape::right_triangle::RightTriangle;

fn main() {
    let rect = Rectangle {
        width: 1.0,
        height: 2.0,
    };
    println!("{}", area(&rect));
    println!("{}", length(&rect));

    let right = RightTriangle {
        width: 3.0,
        height: 8.0,
    };
    println!("{}", area(&right));
    println!("{}", length(&right));

    let line = Line { length: 10.0 };
    println!("{}", length(&line));
}

fn area<T>(x: &T) -> f64
where
    T: CalcArea,
{
    x.calc_area()
}

fn length<T: CalcLength>(x: &T) -> f64 {
    x.calc_length()
}
