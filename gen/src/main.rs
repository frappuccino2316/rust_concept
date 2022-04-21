mod calc;
mod rectangle;
mod right_triangle;

fn main() {
    let rect = rectangle::Rectangle {
        width: 1.0,
        height: 2.0,
    };
    println!("{}", area(&rect));

    let right = right_triangle::RightTriangle {
        width: 3.0,
        height: 8.0,
    };
    println!("{}", area(&right));
}

fn area<T: calc::CalcArea>(x: &T) -> f64 {
    x.calc_area()
}
