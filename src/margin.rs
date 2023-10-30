use float_cmp::F64Margin;

pub struct Margin {}

impl Margin {
    pub fn default_f64() -> F64Margin {
        F64Margin {
            ulps: 2,
            epsilon: 1e-14,
        }
    }
}
