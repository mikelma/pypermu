use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rand::prelude::*;
use rand::distributions::WeightedIndex;

use crate::Vector;

#[doc(hidden)]
pub fn init_mod_pl(py: Python) -> PyResult<&PyModule> {
    let submod = PyModule::new(py, "placket_luce")?;
    submod.add_function(wrap_pyfunction!(sample_pl, submod)?)?;
    PyResult::Ok(submod)
}

#[pyfunction]
pub fn sample_pl(weights: Vec<f32>) -> PyResult<Vector> {
    let n = weights.len();
    let mut sample = Vec::with_capacity(n);

    let mut rng = thread_rng(); 
    let mut distr = WeightedIndex::new(&weights).unwrap();

    (0..n).for_each(|i| { 
        let r = distr.sample(&mut rng);
        sample.push(r);
        // do not update the distribution if the last element has been sampled
        if i < n-1 {
            distr.update_weights(&[(r, &0.0)]).unwrap();
        }
    });

    Ok(sample)
}
