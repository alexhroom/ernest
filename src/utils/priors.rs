pub trait Prior {
    /// rescale a point in unit hypercube space to a point in parameter space for this prior.
    fn rescale(&self, val: f64) -> f64;
}

/// A prior that is uniform over some range.
struct UniformPrior {
    min: f64,
    max: f64
}

impl Prior for UniformPrior {
    fn rescale(&self, val: f64) -> f64 {
        val*(self.max - self.min) + self.min
    }
}
