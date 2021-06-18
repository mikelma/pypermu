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
/// **Note:** The weights vector does not need to be normalized, however, weigths must be positive.
#[pyfunction]
pub fn sample_pl(weights: Vec<f32>, n_samples: usize) -> PyResult<Population> {
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
