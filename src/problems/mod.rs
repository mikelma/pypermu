use pyo3::prelude::*;
// use pyo3::{wrap_pyfunction, wrap_pymodule};

use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod pfsp;
pub mod qap;

#[doc(hidden)]
pub fn init_mod_problems(py: Python) -> PyResult<&PyModule> {
    let module = PyModule::new(py, "problems")?;

    // add qap submodule
    let submod = qap::init_mod_qap(py)?;
    module.add_submodule(submod)?;
    // add pfsp submodule
    let submod = pfsp::init_mod_pfsp(py)?;
    module.add_submodule(submod)?;

    PyResult::Ok(module)
}

/// utility to convert a buffer into a matrix of the specified shape.
#[doc(hidden)]
fn lines2matrix(
    buffer: &mut BufReader<File>,
    n_lines: usize,
    n_elems: usize,
) -> Result<Vec<Vec<usize>>, &str> {
    let mut matrix = vec![Vec::with_capacity(n_elems); n_lines];
    for m_row in matrix.iter_mut() {
        // read the line and split in withespaces
        let mut line = String::new();
        buffer.read_line(&mut line).unwrap();
        let line = line.split_whitespace();
        // parse all numbers from str to usize
        let mut count = 0;
        for str_num in line {
            m_row.push(match str_num.trim().parse() {
                Ok(n) => n,
                Err(_) => return Err("parsing error"),
            });
            count += 1;
        }
        // check if line length is ok
        if count != n_elems {
            return Err("All rows must have the same length as the instance size");
        }
    }
    Ok(matrix)
}
