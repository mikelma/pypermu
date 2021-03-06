use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rayon::prelude::*;

use super::{Population, Vector};

#[doc(hidden)]
pub fn init_mod_utils(py: Python) -> PyResult<&PyModule> {
    let module = PyModule::new(py, "utils")?;

    module.add_submodule(transformations::init_mod_transformations(py)?)?;
    module.add_function(wrap_pyfunction!(borda, module)?)?;
    module.add_function(wrap_pyfunction!(compose, module)?)?;
    module.add_function(wrap_pyfunction!(compose_par, module)?)?;

    PyResult::Ok(module)
}

/// Composes permutations `b` with permutation `c[i]`: `a * b[i]`.
///#[pyfunction]
fn compose_inner(a: Vector, permus: &Population) -> PyResult<Population> {
    let n = a.len();
    let n_perm = permus.len();
    assert_eq!(n, permus[0].len(), "Vector sizes must match");
    let mut out = vec![vec![0usize; n]; n_perm];
    for index in 0..n_perm {
        for i in 0..n {
            out[index][permus[index][i]] = a[i];
        }
    }
    Ok(out)
}

#[pyfunction]
pub fn compose(vec: Vector, pop: Population) -> PyResult<Population> {
    compose_inner(vec, &pop)
}

/// Applies `compose` function to a batch populations. **NOTE**: This function uses
/// parallelisation.
#[pyfunction]
pub fn compose_par(vecs: Population, pops: Vec<Population>) -> PyResult<Vec<Population>> {
    pops.par_iter()
        .zip(vecs)
        .map(|(batch, vec)| compose_inner(vec, batch))
        .collect()
}

/// Returns the Borda (or central) permutation of a list of permutations.
#[pyfunction]
pub fn borda(pop: Population) -> PyResult<Vector> {
    let n = pop[0].len();
    let mut sums: Vec<(usize, usize)> = (0..n).zip(vec![0; n]).collect();

    pop.iter()
        .for_each(|permu| permu.iter().enumerate().for_each(|(i, e)| sums[i].1 += e));

    sums.sort_by_key(|&(_, e)| e);
    for i in 0..n {
        sums[i].1 = i;
    }
    sums.sort_by_key(|&(e, _)| e);
    let (_, res): (Vec<usize>, Vec<usize>) = sums.iter().cloned().unzip();

    Ok(res)
}

pub mod transformations {
    use crate::Population;
    use pyo3::prelude::*;
    use pyo3::wrap_pyfunction;
    // use rayon::prelude::*;

    #[doc(hidden)]
    pub fn init_mod_transformations(py: Python) -> PyResult<&PyModule> {
        let submod = PyModule::new(py, "transformations")?;

        submod.add_function(wrap_pyfunction!(permu2marina, submod)?)?;

        submod.add_function(wrap_pyfunction!(marina2permu, submod)?)?;
        // submod.add_function(wrap_pyfunction!(marina2permu_batch, submod)?)?;

        // submod.add_function(wrap_pyfunction!(permu2inverse_population, submod)?)?;
        // submod.add_function(wrap_pyfunction!(permu2inverse_batch, submod)?)?;
        submod.add_function(wrap_pyfunction!(permu2inverse, submod)?)?;

        PyResult::Ok(submod)
    }

    /// Returns the marina inversion vector representation of the given permutation.
    #[pyfunction]
    pub fn permu2marina(permus: Population) -> PyResult<Population> {
        let n = permus[0].len();
        let n_perm = permus.len();
        let mut outs = vec![vec![0usize; n]; n_perm];
        for index in 0..n_perm {
            for i in 0..n {
                outs[index][i] = permus[index]
                    .iter()
                    .skip(i)
                    .filter(|&&e| permus[index][i] > e)
                    .count();
            }
        }
        Ok(outs)
    }

    #[pyfunction]
    pub fn marina2permu(marinas: Population) -> PyResult<Population> {
        let n = marinas[0].len();
        let n_vecs = marinas.len();
        let mut permus = vec![vec![0usize; n]; n_vecs];

        let e: Vec<usize> = (0..n).collect();
        let mut mask = vec![false; n];
        for i in 0..n_vecs {
            mask = mask.iter().map(|_| false).collect(); // reset the mask
            for j in 0..n {
                permus[i][j] = match e
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| !mask[*i]) // apply the mask to the identity `e`
                    .nth(marinas[i][j])
                {
                    Some((_, v)) => *v,
                    None => panic!("Error converting {:?} to permutation", marinas[i]),
                };
                mask[permus[i][j]] = true;
            }
        }
        Ok(permus)
    }

    /*
    #[pyfunction]
    pub fn marina2permu(marinas: Population) -> PyResult<Population> {
        marina2permu_internal(&marinas)
    }

    #[pyfunction]
    pub fn marina2permu_batch(marinas: Vec<Population>) -> PyResult<Vec<Population>> {
        marinas
            .par_iter()
            .map(|batch| marina2permu(batch))
            .collect()
    }
    */

    #[pyfunction]
    pub fn permu2inverse(permus: Population) -> PyResult<Population> {
        let n = permus[0].len();
        let n_vecs = permus.len();
        let mut inverses: Vec<Vec<usize>> = vec![vec![0usize; n]; n_vecs];
        for index in 0..n_vecs {
            for i in 0..n {
                inverses[index][permus[index][i]] = i;
            }
        }
        Ok(inverses)
    }

    /*
    #[pyfunction]
    pub fn permu2inverse_population(permus: Population) -> PyResult<Population> {
        permu2inverse(&permus)
    }
    */

    /*
    #[pyfunction]
    pub fn permu2inverse_batch(permus: Vec<Population>) -> PyResult<Vec<Population>> {
        permus
            .par_iter()
            .map(|batch| permu2inverse(batch))
            .collect()
    }
    */
}
