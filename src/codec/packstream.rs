mod v1;

use pyo3::basic::CompareOp;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyTuple};
use pyo3::IntoPyObjectExt;

use crate::register_package;

pub(super) fn init_module(m: &Bound<PyModule>, name: &str) -> PyResult<()> {
    let py = m.py();

    m.gil_used(false)?;
    register_package(m, name)?;

    let mod_v1 = PyModule::new(py, "v1")?;
    m.add_submodule(&mod_v1)?;
    v1::init_module(&mod_v1, format!("{name}.v1").as_str())?;

    m.add_class::<Structure>()?;

    Ok(())
}

#[pyclass]
#[derive(Debug)]
pub struct Structure {
    tag: u8,
    #[pyo3(get)]
    fields: Vec<PyObject>,
}

#[pymethods]
impl Structure {
    #[new]
    #[pyo3(signature = (tag, *fields))]
    #[pyo3(text_signature = "(tag, *fields)")]
    fn new(tag: &[u8], fields: Vec<PyObject>) -> PyResult<Self> {
        if tag.len() != 1 {
            return Err(PyErr::new::<PyValueError, _>("tag must be a single byte"));
        }
        let tag = tag[0];
        Ok(Self { tag, fields })
    }

    #[getter(tag)]
    fn read_tag<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &[self.tag])
    }

    #[getter(fields)]
    fn read_fields<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(py, &self.fields)
    }

    fn eq(&self, other: &Self, py: Python<'_>) -> PyResult<bool> {
        if self.tag != other.tag || self.fields.len() != other.fields.len() {
            return Ok(false);
        }
        for (a, b) in self
            .fields
            .iter()
            .map(|e| e.bind(py))
            .zip(other.fields.iter().map(|e| e.bind(py)))
        {
            if !a.eq(b)? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyResult<PyObject> {
        Ok(match op {
            CompareOp::Eq => self.eq(other, py)?.into_py_any(py)?,
            CompareOp::Ne => (!self.eq(other, py)?).into_py_any(py)?,
            _ => py.NotImplemented(),
        })
    }

    fn __hash__(&self, py: Python<'_>) -> PyResult<isize> {
        let mut fields_hash = 0;
        for field in &self.fields {
            fields_hash += field.bind(py).hash()?;
        }
        Ok(fields_hash.wrapping_add(self.tag.into()))
    }
}
