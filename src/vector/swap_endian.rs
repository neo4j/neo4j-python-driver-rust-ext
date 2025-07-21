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

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyInt};
use pyo3::{pyfunction, Bound, PyErr, PyResult};

#[pyfunction]
pub(super) fn swap_endian<'py>(
    type_size: Bound<'py, PyInt>,
    data: Bound<'py, PyBytes>,
) -> PyResult<Bound<'py, PyBytes>> {
    let py = type_size.py();

    let type_size: usize = match type_size.extract::<usize>() {
        Ok(type_size @ 2) | Ok(type_size @ 4) | Ok(type_size @ 8) => type_size,
        _ => {
            return Err(PyErr::new::<PyValueError, _>(format!(
                "Unsupported type size {type_size}",
            )))
        }
    };
    let bytes = &data.as_bytes();
    let len = bytes.len();
    if len % type_size != 0 {
        return Err(PyErr::new::<PyValueError, _>(
            "Data length not a multiple of type_size",
        ));
    }

    PyBytes::new_with(py, bytes.len(), |out| {
        match type_size {
            2 => swap_n::<2>(bytes, out),
            4 => swap_n::<4>(bytes, out),
            8 => swap_n::<8>(bytes, out),
            _ => unreachable!(),
        }
        Ok(())
    })
}

#[inline]
fn swap_n<const N: usize>(src: &[u8], dst: &mut [u8]) {
    // Doesn't technically need to be a function with a const generic, but this
    // allows the compiler to optimize the code better.
    assert_eq!(src.len(), dst.len());
    assert_eq!(src.len() % N, 0);
    for i in (0..src.len()).step_by(N) {
        for j in 0..N {
            dst[i + j] = src[i + N - j - 1];
        }
    }
}
