/// Data structures for inputs
/// Trait to define an object as a prior.
pub trait Prior {
    fn sample(&mut self) -> f64;
    /// randomly sample a point from this prior.

    fn rescale(&mut self, val: f64) -> f64;
    /// rescale a point in unit hypercube space to a point in parameter space for this prior.
}

/// Trait to define an object as a likelihood-restricted prior sampler.
pub trait LRPS {
    fn sample(&mut self, min_likelihood: f64) -> f64;
    /// sample a point with likelihood greater than ``min_likelihood``.
}

