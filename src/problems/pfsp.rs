use pyo3::prelude::*;
use rayon::prelude::*;

use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::{Population, Vector};

#[doc(hidden)]
pub fn init_mod_pfsp(py: Python) -> PyResult<&PyModule> {
    let submod = PyModule::new(py, "pfsp")?;
    submod.add_class::<Pfsp>()?;
    PyResult::Ok(submod)
}

fn evaluate_pfsp(solutions: &Population, matrix: &Vec<Vec<usize>>, n_machines: usize) -> Vector {
    // check if the solution's length matches with the size of the problem
    // create a vector to hold the fitness values and allocate the needed memory beforehand
    let mut fitness_vec = Vec::with_capacity(solutions.len());
    for solution in solutions.iter() {
        let mut tft = 0;
        let mut b = vec![0; n_machines];
        for (job_i, job_n) in solution.iter().enumerate() {
            let mut pt = 0;
            for machine in 0..n_machines {
                if job_i == 0 && machine == 0 {
                    pt = matrix[machine][*job_n];
                } else if job_i > 0 && machine == 0 {
                    pt = b[machine] + matrix[machine][*job_n];
                } else if job_i == 0 && machine > 0 {
                    pt = b[machine - 1] + matrix[machine][*job_n];
                } else if job_i > 0 && machine > 0 {
                    pt = max(b[machine - 1], b[machine]) + matrix[machine][*job_n];
                }
                b[machine] = pt;
            }
            tft += pt;
        }
        fitness_vec.push(tft);
    }
    fitness_vec
}

#[pyclass]
pub struct Pfsp {
    size: usize,
    n_machines: usize,
    matrix: Vec<Vec<usize>>,
}

#[pymethods]
impl Pfsp {
    #[new]
    pub fn load(path: &str) -> PyResult<Pfsp> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        // read lines containing the size of the matrix
        let mut size_str = String::new();
        let _n = reader.read_line(&mut size_str); // ignore first line
        size_str.clear();
        let _n = reader.read_line(&mut size_str); // get sizes line

        // parse instance's sizes (n_jobs and n_machines)
        let mut splitted = size_str.split_whitespace();
        let mut count = 0;
        let mut sizes = vec![]; // n_jobs and n_machines
        while count < 2 {
            if let Some(item) = splitted.next() {
                let num: usize = item.trim().parse()?;
                sizes.push(num);
                count += 1;
            } else {
                panic!("Cannot find size inside instance file");
            }
        }
        // ignore a line
        let _n = reader.read_line(&mut size_str);
        // read the instance matrix
        let matrix = super::lines2matrix(&mut reader, sizes[1], sizes[0]).unwrap();
        Ok(Pfsp {
            size: sizes[0],
            n_machines: sizes[1],
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
        Ok(evaluate_pfsp(&solutions, &self.matrix, self.n_machines))
    }

    pub fn evaluate_batch(&self, solutions: Vec<Population>) -> PyResult<Vec<Vector>> {
        solutions
            .par_iter()
            .map(|batch| Ok(evaluate_pfsp(batch, &self.matrix, self.n_machines)))
            .collect()
    }
}
