use pyo3::prelude::*;

pub mod placket_luce;

#[doc(hidden)]
pub fn init_mod_distributions(py: Python) -> PyResult<&PyModule> {
    let module = PyModule::new(py, "distributions")?;
    let submod = placket_luce::init_mod_pl(py)?;
    module.add_submodule(submod)?;

    PyResult::Ok(module)
}
