use crate::utils::lrps::LRPS;

struct MultiNest;

impl LRPS for MultiNest{
    fn sample(&mut self, min_likelihood: f64) -> (Vec<f64>, f64) {
        todo!()
    }
}
