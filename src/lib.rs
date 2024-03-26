pub mod args;
pub mod auditor;
pub mod config;
pub mod errors;

use pyo3::prelude::*;

#[pymodule]
fn auditor_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<auditor::Auditor>()?;
    Ok(())
}
