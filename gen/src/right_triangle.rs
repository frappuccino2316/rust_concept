use crate::calc::CalcArea;

pub struct RightTriangle {
    pub width: f64,
    pub height: f64,
}

impl CalcArea for RightTriangle {
    fn calc_area(&self) -> f64 {
        self.width * self.height * 0.5
    }
}
