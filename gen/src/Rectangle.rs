use crate::calc::CalcArea;

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl CalcArea for Rectangle {
    fn calc_area(&self) -> f64 {
        self.width * self.height
    }
}
