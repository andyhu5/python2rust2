use std::{collections::HashMap};

use pyo3::{exceptions::PyValueError, prelude::*, types::PyDict};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A simple function that returns a list

#[pyfunction]
fn sum_as_list(numbers: Vec<usize>) -> PyResult<Vec<usize>> {
    Ok(numbers.iter().map(|x| x + 1).collect())
}

/// A simple function that returns a map
#[pyfunction]
fn sum_as_map(numbers: HashMap<usize,usize>) -> PyResult<Py<PyDict>> {
    Python::with_gil(|py| {
        let map = PyDict::new(py);
        for (i, (_key, value)) in numbers.iter().enumerate() {
            map.set_item(i, value + 1)?;
        }
        Ok(map.into())
    })
}


#[pyfunction]
fn hello() -> PyResult<String> {
    Ok("Hello, world!".to_string())
}

#[pyclass]
struct Counter {
    value:i32,
}

#[pymethods]
impl Counter {
    #[new]
    fn new(value:i32) -> Self {
        Counter { value }
    }
    fn increment(&mut self) {
        self.value += 1;
    }
    fn get_value(&self) -> i32 {
        self.value
    }
}

#[pyfunction]
fn divide(a: f64, b: f64) -> PyResult<f64> {
    if b == 0.0 {
        return Err(PyValueError::new_err("division by zero"))
    } else {
        Ok(a / b)
    }
}

#[pyfunction]
fn call_python_function(value:i32)-> PyResult<i32> {
    Python::with_gil(|py| {
        let math = py.import("math")?;
        let result = math.getattr("floor")?.call1((value,))?.extract::<i32>()?;
        Ok(result)
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn newbee(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_list, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_map, m)?)?;
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    m.add_class::<Counter>()?;
    m.add_function(wrap_pyfunction!(divide, m)?)?;
    m.add_function(wrap_pyfunction!(call_python_function, m)?)?;
    Ok(())
}
