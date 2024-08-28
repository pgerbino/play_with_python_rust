// we use mod keyword because we want to create a module
// a module is a collection of items: functions, structs, traits, impl blocks, and even other modules
mod more_polars;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use pyo3_polars::PyDataFrame;
use std::cell::Cell;

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

#[pymodule]
pub fn decorator(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyCounter>()?;
    Ok(())
}
