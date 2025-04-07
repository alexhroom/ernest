/// The main nested sampler function.
use rand::Rng

fn nested_sampler<S: LPRS>(priors: Vec<Box<Prior>>, n_live: usize, sampler: &mut S) -> (f64, Vec<f64>, Vec<f64>, f64) {
    /// Run the nested sampler for a given likelihood-restricted prior sampler 
    /// and set of priors.

    /// generate initial live points
    /// and likelihoods

    /* enter main loop:
    /// get minimum likelihood
    /// get log weight, update evidence, information, width

    /// run the LPRS
 
    /// replace minimum likelihood point with new point

    end main loop */

    /// add remaining live points to logZ and H
}
