// Copyright (c) "Neo4j"
// Neo4j Sweden AB [https://neo4j.com]
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod v1;

use pyo3::basic::CompareOp;
use pyo3::exceptions::{PyIndexError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::{IntoPyObjectExt, PyTraverseError, PyVisit};

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
    fields: Vec<Py<PyAny>>,
}

impl Structure {
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

    fn compute_index(&self, index: isize) -> PyResult<usize> {
        Ok(if index < 0 {
            self.fields
                .len()
                .checked_sub(-index as usize)
                .ok_or_else(|| PyErr::new::<PyIndexError, _>("field index out of range"))?
        } else {
            let index = index as usize;
            if index >= self.fields.len() {
                return Err(PyErr::new::<PyIndexError, _>("field index out of range"));
            }
            index
        })
    }
}

#[pymethods]
impl Structure {
    #[new]
    #[pyo3(signature = (tag, *fields))]
    #[pyo3(text_signature = "(tag, *fields)")]
    fn new(tag: &[u8], fields: Vec<Py<PyAny>>) -> PyResult<Self> {
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

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        let mut args = format!(r"b'{}'", self.tag as char);
        self.fields.iter().try_for_each(|field| {
            let repr = field.bind(py).repr()?;
            args.push_str(", ");
            args.push_str(&repr.to_cow()?);
            Ok::<_, PyErr>(())
        })?;
        Ok(format!("Structure({args})"))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyResult<Py<PyAny>> {
        Ok(match op {
            CompareOp::Eq => self.eq(other, py)?.into_py_any(py)?,
            CompareOp::Ne => (!self.eq(other, py)?).into_py_any(py)?,
            _ => py.NotImplemented(),
        })
    }

    fn __len__(&self) -> usize {
        self.fields.len()
    }

    fn __getitem__(&self, index: isize, py: Python<'_>) -> PyResult<Py<PyAny>> {
        Ok(self.fields[self.compute_index(index)?].clone_ref(py))
    }

    fn __setitem__(&mut self, index: isize, value: Py<PyAny>) -> PyResult<()> {
        let index = self.compute_index(index)?;
        self.fields[index] = value;
        Ok(())
    }

    fn __traverse__(&self, visit: PyVisit<'_>) -> Result<(), PyTraverseError> {
        for field in &self.fields {
            visit.call(field)?;
        }
        Ok(())
    }

    fn __clear__(&mut self) {
        self.fields.clear();
    }
}
