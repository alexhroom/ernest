pub mod multinest;

/// Trait to define an object as a likelihood-restricted prior sampler.
pub trait LRPS {
    /// sample a point with likelihood greater than ``min_likelihood`` in the unit hypercube.
    fn sample(&mut self, min_likelihood: f64) -> (Vec<f64>, f64);
}
