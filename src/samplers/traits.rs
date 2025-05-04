/// Data structures for inputs
/// Trait to define an object as a prior.
pub trait Prior {
    /// randomly sample a point from this prior.
    fn sample(&self) -> f64;

    /// rescale a point in unit hypercube space to a point in parameter space for this prior.
    fn rescale(&self, val: f64) -> f64;
}

/// Trait to define an object as a likelihood-restricted prior sampler.
pub trait LRPS {
    /// sample a point with likelihood greater than ``min_likelihood`` in the unit hypercube.
    fn sample(&mut self, min_likelihood: f64) -> (Vec<f64>, f64);
}
