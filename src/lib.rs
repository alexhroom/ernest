/// The main nested sampler function.
use crate::samplers::traits::{Prior, LRPS};
use rand::Rng;

pub mod samplers;

/// Run the nested sampler for a given likelihood-restricted prior sampler
/// and set of priors.
fn nested_sampler<S: LRPS>(
    priors: Vec<Box<dyn Prior>>,
    n_live: usize,
    log_likelihood: &dyn Fn(Vec<f64>) -> f64,
    tolerance: f64,
    sampler: &mut S,
) -> (f64, Vec<Vec<f64>>, Vec<f64>, f64) {
    let mut rng = rand::rng();
    let n_params = priors.len();

    // choose live points in the unit hypercube
    // and transform them to points in parameter space
    let mut live_hypercube: Vec<Vec<f64>> = (0..n_live)
        .map(|_| (0..n_params).map(|_| rng.random::<f64>()).collect())
        .collect();
    let mut param_vals: Vec<Vec<f64>> = live_hypercube
        .clone()
        .into_iter()
        .map(|v| {
            v.into_iter()
                .enumerate()
                .map(|(i, x)| priors[i].rescale(x))
                .collect::<Vec<f64>>()
        })
        .collect();
    // calculate log-likelihood for each point in parameter space
    let mut log_likelihoods: Vec<f64> =
        param_vals.clone().into_iter().map(log_likelihood).collect();

    // create initial values of outputs
    // initial value of Z is 0, so initial value of log_Z is -inf
    let mut log_z = -f64::INFINITY;
    let mut last_log_z: f64;
    let mut all_param_vals: Vec<Vec<f64>> = Vec::new();
    let mut all_log_likelihoods: Vec<f64> = Vec::new();
    let mut entropy = 0.0;
    let mut last_entropy: f64;
    // log of width of prior volume
    let mut log_prior_width = (1.0 - (-1.0 / n_live as f64).exp()).ln();
    let mut tol = f64::INFINITY;

    let mut iteration = 1;
    while tol > tolerance {
        iteration += 1;
        // enter main loop:
        let (least_index, least_likelihood): (usize, &f64) = log_likelihoods
            .iter()
            .enumerate()
            .min_by(|(_, val1), (_, val2)| {
                val1.partial_cmp(val2)
                    .expect("log_likelihoods contains a NaN")
            })
            .unwrap();

        all_param_vals.push(param_vals[least_index].clone());
        all_log_likelihoods.push(*least_likelihood);

        // weight is likelihood * prior_width, so logs make this an addition
        let log_weight = least_likelihood + log_prior_width;

        // update evidence, information, width
        last_log_z = log_z;
        log_z = (log_z.exp() + log_weight.exp()).ln();
        last_entropy = entropy;
        entropy = (log_weight - log_z).exp() * least_likelihood
            + (last_log_z - log_z).exp() * (last_entropy + last_log_z)
            - log_z;
        log_prior_width -= (1 / n_live) as f64;

        // get new sampled point and replace old point in live vectors
        let (new_point, new_point_log_l) = sampler.sample(*least_likelihood);

        live_hypercube[least_index] = new_point.clone();
        param_vals[least_index] = new_point
            .into_iter()
            .enumerate()
            .map(|(i, x)| priors[i].rescale(x))
            .collect();
        log_likelihoods[least_index] = new_point_log_l;

        let max_likelihood = log_likelihoods
            .iter()
            .max_by(|x, y| x.partial_cmp(y).expect("log_likelihoods contains a NaN"))
            .unwrap();
        tol = (log_z.exp() - max_likelihood * (iteration / n_live) as f64).ln() - log_z;
    }
    // end main loop

    // add remaining live points to logZ and H
    (log_z, all_param_vals, all_log_likelihoods, entropy)
}
