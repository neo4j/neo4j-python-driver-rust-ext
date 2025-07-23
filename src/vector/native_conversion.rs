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

use pyo3::exceptions::{PyOverflowError, PyTypeError};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyFloat, PyInt, PyList};

// =================
// ====== F64 ======
// =================

#[pyfunction]
pub(super) fn vec_f64_from_native<'py>(data: Bound<'py, PyAny>) -> PyResult<Bound<'py, PyBytes>> {
    let py = data.py();

    let data_iter = data.try_iter()?;
    let mut bytes = Vec::with_capacity(data_iter.size_hint().0.saturating_mul(size_of::<f64>()));
    for value in data_iter {
        let value = vec_value_as_f64(value?)?;
        bytes.extend(&f64::to_be_bytes(value));
    }
    Ok(PyBytes::new(py, &bytes))
}

fn vec_value_as_f64(value: Bound<PyAny>) -> PyResult<f64> {
    fn make_error<T>(value: &Bound<PyAny>) -> PyResult<T> {
        Err(PyErr::new::<PyTypeError, _>(format!(
            "Cannot convert value to f64, expected float, got {}.",
            value.get_type().name()?
        )))
    }

    value
        .downcast::<PyFloat>()
        .or_else(|_| make_error(&value))?
        .extract()
        .or_else(|_| make_error(&value))
}

#[pyfunction]
pub(super) fn vec_f64_to_native<'py>(data: Bound<'py, PyBytes>) -> PyResult<Bound<'py, PyList>> {
    const DATA_SIZE: usize = size_of::<f64>();
    let py = data.py();
    PyList::new(
        py,
        data.as_bytes().chunks(DATA_SIZE).map(|chunk| {
            let value = f64::from_be_bytes(
                chunk
                    .try_into()
                    .expect("bytes size is not multiple of type size"),
            );
            PyFloat::new(py, value)
        }),
    )
}

// =================
// ====== F32 ======
// =================

#[pyfunction]
pub(super) fn vec_f32_from_native<'py>(data: Bound<'py, PyAny>) -> PyResult<Bound<'py, PyBytes>> {
    let py = data.py();

    let data_iter = data.try_iter()?;
    let mut bytes = Vec::with_capacity(data_iter.size_hint().0.saturating_mul(size_of::<f32>()));
    for value in data_iter {
        let value = vec_value_as_f32(value?)?;
        bytes.extend(&f32::to_be_bytes(value));
    }
    Ok(PyBytes::new(py, &bytes))
}

fn vec_value_as_f32(value: Bound<PyAny>) -> PyResult<f32> {
    fn make_error<T>(value: &Bound<PyAny>) -> PyResult<T> {
        Err(PyErr::new::<PyTypeError, _>(format!(
            "Cannot convert value to f32, expected float, got {}.",
            value.get_type().name()?
        )))
    }

    value
        .downcast::<PyFloat>()
        .or_else(|_| make_error(&value))?
        .extract()
        .or_else(|_| make_error(&value))
}

#[pyfunction]
pub(super) fn vec_f32_to_native<'py>(data: Bound<'py, PyBytes>) -> PyResult<Bound<'py, PyList>> {
    const DATA_SIZE: usize = size_of::<f32>();
    let py = data.py();
    PyList::new(
        py,
        data.as_bytes().chunks(DATA_SIZE).map(|chunk| {
            let value = f32::from_be_bytes(
                chunk
                    .try_into()
                    .expect("bytes size is not multiple of type size"),
            );
            PyFloat::new(py, value.into())
        }),
    )
}

// =================
// ====== I64 ======
// =================

#[pyfunction]
pub(super) fn vec_i64_from_native<'py>(data: Bound<'py, PyAny>) -> PyResult<Bound<'py, PyBytes>> {
    let py = data.py();

    let data_iter = data.try_iter()?;
    let mut bytes = Vec::with_capacity(data_iter.size_hint().0.saturating_mul(size_of::<i64>()));
    for value in data_iter {
        let value = vec_value_as_i64(value?)?;
        bytes.extend(&i64::to_be_bytes(value));
    }
    Ok(PyBytes::new(py, &bytes))
}

fn vec_value_as_i64(value: Bound<PyAny>) -> PyResult<i64> {
    fn make_error<T>(value: &Bound<PyAny>) -> PyResult<T> {
        Err(PyErr::new::<PyTypeError, _>(format!(
            "Cannot convert value to i64, expected int, got {}.",
            value.get_type().name()?
        )))
    }

    let py = value.py();

    let value = value.downcast::<PyInt>().or_else(|_| make_error(&value))?;
    if value.lt(PyInt::new(py, i64::MIN))? || value.gt(PyInt::new(py, i64::MAX))? {
        return Err(PyErr::new::<PyOverflowError, _>(format!(
            "Value {} is out of range for i64: [-9223372036854775808, 9223372036854775807]",
            value.str()?
        )));
    }
    value.extract().or_else(|_| make_error(value))
}

#[pyfunction]
pub(super) fn vec_i64_to_native<'py>(data: Bound<'py, PyBytes>) -> PyResult<Bound<'py, PyList>> {
    const DATA_SIZE: usize = size_of::<i64>();
    let py = data.py();
    PyList::new(
        py,
        data.as_bytes().chunks(DATA_SIZE).map(|chunk| {
            let value = i64::from_be_bytes(
                chunk
                    .try_into()
                    .expect("bytes size is not multiple of type size"),
            );
            PyInt::new(py, value)
        }),
    )
}

// =================
// ====== I32 ======
// =================

#[pyfunction]
pub(super) fn vec_i32_from_native<'py>(data: Bound<'py, PyAny>) -> PyResult<Bound<'py, PyBytes>> {
    let py = data.py();

    let data_iter = data.try_iter()?;
    let mut bytes = Vec::with_capacity(data_iter.size_hint().0.saturating_mul(size_of::<i32>()));
    for value in data_iter {
        let value = vec_value_as_i32(value?)?;
        bytes.extend(&i32::to_be_bytes(value));
    }
    Ok(PyBytes::new(py, &bytes))
}

fn vec_value_as_i32(value: Bound<PyAny>) -> PyResult<i32> {
    fn make_error<T>(value: &Bound<PyAny>) -> PyResult<T> {
        Err(PyErr::new::<PyTypeError, _>(format!(
            "Cannot convert value to i32, expected int, got {}.",
            value.get_type().name()?
        )))
    }

    let py = value.py();

    let value = value.downcast::<PyInt>().or_else(|_| make_error(&value))?;
    if value.lt(PyInt::new(py, i32::MIN))? || value.gt(PyInt::new(py, i32::MAX))? {
        return Err(PyErr::new::<PyOverflowError, _>(format!(
            "Value {} is out of range for i32: [-2147483648, 2147483647]",
            value.str()?
        )));
    }
    value.extract().or_else(|_| make_error(value))
}

#[pyfunction]
pub(super) fn vec_i32_to_native<'py>(data: Bound<'py, PyBytes>) -> PyResult<Bound<'py, PyList>> {
    const DATA_SIZE: usize = size_of::<i32>();
    let py = data.py();
    PyList::new(
        py,
        data.as_bytes().chunks(DATA_SIZE).map(|chunk| {
            let value = i32::from_be_bytes(
                chunk
                    .try_into()
                    .expect("bytes size is not multiple of type size"),
            );
            PyInt::new(py, value)
        }),
    )
}

// =================
// ====== I16 ======
// =================

#[pyfunction]
pub(super) fn vec_i16_from_native<'py>(data: Bound<'py, PyAny>) -> PyResult<Bound<'py, PyBytes>> {
    let py = data.py();

    let data_iter = data.try_iter()?;
    let mut bytes = Vec::with_capacity(data_iter.size_hint().0.saturating_mul(size_of::<i16>()));
    for value in data_iter {
        let value = vec_value_as_i16(value?)?;
        bytes.extend(&i16::to_be_bytes(value));
    }
    Ok(PyBytes::new(py, &bytes))
}

fn vec_value_as_i16(value: Bound<PyAny>) -> PyResult<i16> {
    fn make_error<T>(value: &Bound<PyAny>) -> PyResult<T> {
        Err(PyErr::new::<PyTypeError, _>(format!(
            "Cannot convert value to i16, expected int, got {}.",
            value.get_type().name()?
        )))
    }

    let py = value.py();

    let value = value.downcast::<PyInt>().or_else(|_| make_error(&value))?;
    if value.lt(PyInt::new(py, i16::MIN))? || value.gt(PyInt::new(py, i16::MAX))? {
        return Err(PyErr::new::<PyOverflowError, _>(format!(
            "Value {} is out of range for i16: [-32768, 32767]",
            value.str()?
        )));
    }
    value.extract().or_else(|_| make_error(value))
}

#[pyfunction]
pub(super) fn vec_i16_to_native<'py>(data: Bound<'py, PyBytes>) -> PyResult<Bound<'py, PyList>> {
    const DATA_SIZE: usize = size_of::<i16>();
    let py = data.py();
    PyList::new(
        py,
        data.as_bytes().chunks(DATA_SIZE).map(|chunk| {
            let value = i16::from_be_bytes(
                chunk
                    .try_into()
                    .expect("bytes size is not multiple of type size"),
            );
            PyInt::new(py, value)
        }),
    )
}

// ================
// ====== I8 ======
// ================

#[pyfunction]
pub(super) fn vec_i8_from_native<'py>(data: Bound<'py, PyAny>) -> PyResult<Bound<'py, PyBytes>> {
    let py = data.py();

    let data_iter = data.try_iter()?;
    let mut bytes = Vec::with_capacity(data_iter.size_hint().0.saturating_mul(size_of::<i8>()));
    for value in data_iter {
        let value = vec_value_as_i8(value?)?;
        bytes.extend(&i8::to_be_bytes(value));
    }
    Ok(PyBytes::new(py, &bytes))
}

fn vec_value_as_i8(value: Bound<PyAny>) -> PyResult<i8> {
    fn make_error<T>(value: &Bound<PyAny>) -> PyResult<T> {
        Err(PyErr::new::<PyTypeError, _>(format!(
            "Cannot convert value to i8, expected int, got {}.",
            value.get_type().name()?
        )))
    }

    let py = value.py();

    let value = value.downcast::<PyInt>().or_else(|_| make_error(&value))?;
    if value.lt(PyInt::new(py, i8::MIN))? || value.gt(PyInt::new(py, i8::MAX))? {
        return Err(PyErr::new::<PyOverflowError, _>(format!(
            "Value {} is out of range for i8: [-128, 127]",
            value.str()?
        )));
    }
    value.extract().or_else(|_| make_error(value))
}

#[pyfunction]
pub(super) fn vec_i8_to_native<'py>(data: Bound<'py, PyBytes>) -> PyResult<Bound<'py, PyList>> {
    const DATA_SIZE: usize = size_of::<i8>();
    let py = data.py();
    PyList::new(
        py,
        data.as_bytes().chunks(DATA_SIZE).map(|chunk| {
            let value = i8::from_be_bytes(
                chunk
                    .try_into()
                    .expect("bytes size is not multiple of type size"),
            );
            PyInt::new(py, value)
        }),
    )
}
