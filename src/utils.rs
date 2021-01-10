pub mod extensions {
    pub trait IsZero {
        fn is_zero(&self) -> bool;
    }

    impl IsZero for f64 {
        fn is_zero(&self) -> bool {
            self.abs() < 0.000000001
        }
    }
    pub trait MultEsc {
        fn mult_esc(self, val: f64) -> Vec<f64>;
    }

    impl MultEsc for Vec<f64> {
        fn mult_esc(self, val: f64) -> Vec<f64> {
            let mut res = vec![0.0;self.len()];
            for v in 0..res.len() {
                res[v] = self[v] * val;
            }
            res
        }
    }
    pub trait SumVecEq {
        fn sum_vec_eq(self, other: Vec<f64>) -> Vec<f64>;
    }

    impl SumVecEq for Vec<f64> {
        fn sum_vec_eq(self, other: Vec<f64>) -> Vec<f64> {
            let mut res = vec![0.0;self.len()];
            for v in 0..res.len() {
                res[v] = self[v] + other[v];
            }
            res
        }
    }
}
