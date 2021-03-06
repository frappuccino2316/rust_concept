use crate::shape::calc::CalcLength;

pub struct Line {
    pub length: f64,
}

impl CalcLength for Line {
    fn calc_length(&self) -> f64 {
        self.length
    }
}
