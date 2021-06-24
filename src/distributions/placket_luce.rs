use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rand::prelude::*;
use rand::distributions::WeightedIndex;

use crate::{Population, Vector};

#[doc(hidden)]
pub fn init_mod_pl(py: Python) -> PyResult<&PyModule> {
    let submod = PyModule::new(py, "placket_luce")?;
    submod.add_function(wrap_pyfunction!(sample_pl, submod)?)?;
    PyResult::Ok(submod)
}

/// Samples a given number of samples from a weights vector of a Placket-Luce distribution.
///
/// **Note:** The weights vector do not need to be normalized, however, weigths must be positive.
///
/// *See*: https://docs.rs/rand/0.8.4/rand/seq/trait.SliceRandom.html#tymethod.choose_multiple_weighted
#[pyfunction]
pub fn sample_pl(weights: Vec<f32>, n_samples: usize) -> PyResult<Population> {
    let n = weights.len();
    let mut samples = Vec::with_capacity(n_samples);
    let mut rng = thread_rng(); 

    let choices: Vec<(usize, f64)> = weights.iter().map(|n| *n as f64).enumerate().collect();
    (0..n_samples).for_each(|_| {
        samples.push(choices.choose_multiple_weighted(&mut rng, n, |item| item.1).unwrap().map(|s| s.0).collect::<Vec<_>>());
    });

    Ok(samples)
}

/// Samples a given number of samples from a weights vector of a Placket-Luce distribution.
/// **Note:** The weights vector does not need to be normalized, however, weigths must be positive.
/*
#[pyfunction]
pub fn sample_pl_old(weights: Vec<f32>, n_samples: usize) -> PyResult<Population> {
    let n = weights.len();
    let mut samples = Vec::with_capacity(n_samples);

    let mut rng = thread_rng(); 

    let orig_distr = WeightedIndex::new(&weights).unwrap();

    (0..n_samples).for_each(|_| {
        let mut distr = orig_distr.clone();
        let mut sample = Vec::with_capacity(n);

        (0..n).for_each(|i| { 
            let r = distr.sample(&mut rng);
            sample.push(r);
            // do not update the distribution if the last element has been sampled
            if i < n-1 {
                distr.update_weights(&[(r, &0.0)]).unwrap();
            }
        });
        
        samples.push(sample);
    });

    Ok(samples)
}
*/
