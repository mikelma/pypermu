use pyo3::prelude::*;
use rayon::prelude::*;

use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::{Population, Vector};

#[doc(hidden)]
pub fn init_mod_lop(py: Python) -> PyResult<&PyModule> {
    let submod = PyModule::new(py, "lop")?;
    submod.add_class::<Lop>()?;
    PyResult::Ok(submod)
}

fn evaluate_lop(solutions: &Population, matrix: &[Vec<usize>]) -> Vector {
    let mut fitness_vec = Vec::with_capacity(solutions.len());
    let n = matrix.len();
    /*
    // `for` version
    for solution in solutions {
        let mut sum = 0;
        for (i, sig_i) in solution.iter().enumerate().take(n-1) {
            for sig_j in solution.iter().skip(i+1) {
                sum += matrix[*sig_i][*sig_j];
            }
        }
        fitness_vec.push(sum);
    }
    */
    solutions.iter().for_each(|solution| {
        fitness_vec.push(solution.iter()
            .enumerate()
            .take(n-1)
            .map(|(i, sig_i)| {
                solution.iter()
                    .skip(i+1)
                    .map(|sig_j| {
                        matrix[*sig_i][*sig_j]
                    }).sum::<usize>()
            }).sum::<usize>()
        );
    });

    fitness_vec
}

#[pyclass]
pub struct Lop {
    size: usize,
    matrix: Vec<Vec<usize>>,
}

#[pymethods]
impl Lop {

    #[new]
    pub fn load(path: &str) -> PyResult<Lop> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        // read and parse the line containing the size of the matrix
        let mut size_str = String::new();
        let _n = reader.read_line(&mut size_str);
        let size: usize = size_str.trim().parse()?;

        // read the instance matrix
        let matrix = super::lines2matrix(&mut reader, size, size).unwrap();

        Ok(Lop {
            size,
            matrix,
        })
    }

    pub fn evaluate(&self, solutions: Population) -> PyResult<Vector> {
        // check if the solution's length matches with the size of the problem
        assert_eq!(
            solutions[0].len(),
            self.size,
            "instance and solution sizes must match"
        );
        Ok(evaluate_lop(&solutions, &self.matrix))
    }

    pub fn evaluate_par(&self, solutions: Vec<Population>) -> PyResult<Vec<Vector>> {
        solutions
            .par_iter()
            .map(|batch| Ok(evaluate_lop(batch, &self.matrix)))
            .collect()
    }
}

