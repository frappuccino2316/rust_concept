use crate::shape::calc::CalcArea;
use crate::shape::calc::CalcLength;

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl CalcArea for Rectangle {
    fn calc_area(&self) -> f64 {
        self.width * self.height
    }
}

impl CalcLength for Rectangle {
    fn calc_length(&self) -> f64 {
        (self.width + self.height) * 2.0
    }
}
