// we use mod keyword because we want to create a module
// a module is a collection of items: functions, structs, traits, impl blocks, and even other modules
mod more_polars;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use pyo3_polars::PyDataFrame;
use std::cell::Cell;
// import polars
use polars::prelude::*;
use pyo3_polars::*;

/// A function decorator that keeps track how often it is called.
///
/// It otherwise doesn't do anything special.
#[pyclass(name = "Counter")]
pub struct PyCounter {
    // Keeps track of how many calls have gone through.
    //
    // See the discussion at the end for why `Cell` is used.
    count: Cell<u64>,

    // This is the actual function being wrapped.
    wraps: Py<PyAny>,
}

#[pymethods]
impl PyCounter {
    // Note that we don't validate whether `wraps` is actually callable.
    //
    // While we could use `PyAny::is_callable` for that, it has some flaws:
    //    1. It doesn't guarantee the object can actually be called successfully
    //    2. We still need to handle any exceptions that the function might raise
    #[new]
    fn __new__(wraps: Py<PyAny>) -> Self {
        PyCounter {
            count: Cell::new(0),
            wraps,
        }
    }

    #[getter]
    fn count(&self) -> u64 {
        self.count.get()
    }

    #[pyo3(signature = (*args, **kwargs))]
    fn __call__(
        &self,
        py: Python<'_>,
        args: &Bound<'_, PyTuple>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Py<PyAny>> {
        let old_count = self.count.get();
        let new_count = old_count + 1;
        self.count.set(new_count);
        let name = self.wraps.getattr(py, "__name__")?;

        println!("{} has been called {} time(s).", name, new_count);

        // After doing something, we finally forward the call to the wrapped function
        let ret = self.wraps.call_bound(py, args, kwargs)?;

        // We could do something with the return value of
        // the function before returning it
        Ok(ret)
    }

    // this method needs to be part of the pyo3 module
    // it should be a gil reference not pure rust code
    // this is because we are being called from Python code
    // pub fn add(&self, py: Python, df_a: PyDataFrame, df_b: PyDataFrame) -> PyDataFrame {
    //     // convert the PyDataFrame to a DataFrame
    //     let df_a = df_a.as_ref();
    //     let df_b = df_b.as_ref();
    //     let mask = column("Factor").eq("A");
    //     let filtered_df_b = df_a.filter(column("name").is_in(df_b.column("name").unwrap()));
    //     pyo3_polars::PyDataFrame(filtered_df_b)
    // }
    
}

#[pyfunction]
fn add_two_numbers(a: i64, b: i64) -> i64 {
    a + b
}

#[pyfunction]
#[pyo3 (signature=(**kwds))]
// This function takes an optional dictionary of keyword arguments and prints each key-value pair.
fn num_kwds(kwds: Option<&Bound<'_, PyDict>>) -> PyResult<()> {
    // Check if the `kwds` argument is not `None`.
    if let Some(kwds) = kwds {
        // Iterate over each key-value pair in the dictionary.
        for (key, value) in kwds.iter() {
            // Print the key and its corresponding value.
            println!("{}: {}", key, value);
        }
    }
    // Return `Ok(())` to indicate that the function executed successfully.
    Ok(())
}

// create a pyfunction that takes a polars DataFrame and returns it
#[pyfunction]
fn from_polars(pydf:PyDataFrame) -> PyResult<PyDataFrame>  {
    // let df = df.extract::<DataFrame>(py)?;
    Ok(pydf)
}

#[pymodule]
pub fn my_pyo3_project(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyCounter>()?;
    module.add_function(wrap_pyfunction!(add_two_numbers, module)?)?;
    module.add_function(wrap_pyfunction!(num_kwds, module)?)?;
    module.add_function(wrap_pyfunction!(from_polars, module)?)?;
    Ok(())
}
