use pyo3::prelude::*;

mod problems;
mod utils;

/// Python module for permutations implemented in rust.
#[pymodule]
fn pypermu(py: Python, m: &PyModule) -> PyResult<()> {
    let problems_submod = problems::init_mod_problems(py)?;
    m.add_submodule(problems_submod)?;

    let utils_submod = utils::init_mod_utils(py)?;
    m.add_submodule(utils_submod)?;

    Ok(())
}
