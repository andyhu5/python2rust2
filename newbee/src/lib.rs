use pyo3::{exceptions::PyValueError, prelude::*};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
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
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    m.add_class::<Counter>()?;
    m.add_function(wrap_pyfunction!(divide, m)?)?;
    m.add_function(wrap_pyfunction!(call_python_function, m)?)?;
    Ok(())
}
