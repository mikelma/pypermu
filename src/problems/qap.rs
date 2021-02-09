use pyo3::prelude::*;
// use pyo3::wrap_pyfunction;

use super::lines2matrix;

use std::fs::File;
use std::io::{BufRead, BufReader};

#[doc(hidden)]
pub fn init_mod_qap(py: Python) -> PyResult<&PyModule> {
    let submod = PyModule::new(py, "qap")?;
    submod.add_class::<Qap>()?;
    PyResult::Ok(submod)
}

#[pyclass]
pub struct Qap {
    size: usize,
    dist: Vec<Vec<usize>>,
    flow: Vec<Vec<usize>>,
}

#[pymethods]
impl Qap {
    #[new]
    pub fn from(instance: &str) -> PyResult<Qap> {
        let file = File::open(instance)?;
        let mut reader = BufReader::new(file);
        // get instance's size
        let mut size_str = String::new();
        let _n = reader.read_line(&mut size_str); // Get size
        let size: usize = size_str.trim().parse()?;
        // read matrixes
        let dist = lines2matrix(&mut reader, size, size).unwrap();
        let flow = lines2matrix(&mut reader, size, size).unwrap();

        PyResult::Ok(Qap { size, dist, flow })
    }
    /*
    #[getter(size)]
    pub fn get_size(&self) -> PyResult<usize> {
        Ok(self.size)
    }

    #[setter(size)]
    pub fn set_size(&mut self, s: usize) -> PyResult<()> {
        self.size = s;
        Ok(())
    }

    #[getter(distances)]
    pub fn get_dist(&self) -> PyResult<&Vec<Vec<usize>>> {
        Ok(&self.dist)
    }

    #[setter(distances)]
    pub fn set_dist(&mut self, d: Vec<Vec<usize>>) -> PyResult<()> {
        self.dist = d;
        Ok(())
    }

    #[getter(flows)]
    pub fn get_flow(&self) -> PyResult<PyRef<Vec<Vec<usize>>>> {
        Ok(&self.flow)
    }

    #[setter(flows)]
    pub fn set_flow(&mut self, f: Vec<Vec<usize>>) -> PyResult<()> {
        self.flow = f;
        Ok(())
    }
    */

    pub fn evaluate(&self, solutions: Vec<Vec<usize>>) -> PyResult<Vec<usize>> {
        // check if the solution's length matches with the size of the problem
        assert_eq!(
            solutions[0].len(),
            self.size,
            "instance and solution sizes must match"
        );
        let mut fitness_vec = vec![];
        for solution in solutions.iter() {
            let mut f = 0;
            for i in 0..self.size {
                for j in 0..self.size {
                    let dist_ab = self.dist[i][j];
                    let flow_ab = self.flow[solution[i]][solution[j]];

                    f += dist_ab * flow_ab;
                }
            }
            fitness_vec.push(f);
        }
        PyResult::Ok(fitness_vec)
    }
}
