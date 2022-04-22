use crate::shape::calc::CalcArea;
use crate::shape::calc::CalcLength;

pub struct RightTriangle {
    pub width: f64,
    pub height: f64,
}

impl CalcArea for RightTriangle {
    fn calc_area(&self) -> f64 {
        self.width * self.height * 0.5
    }
}

impl CalcLength for RightTriangle {
    fn calc_length(&self) -> f64 {
        self.width + self.height + (self.width.powi(2) + self.height.powi(2)).sqrt()
    }
}
