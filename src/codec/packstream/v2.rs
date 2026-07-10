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

pub(super) mod extension;

use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyBytes, PyDict};
use pyo3::wrap_pyfunction;

use crate::register_package;

const UUID: u8 = 0xE0;

#[pyfunction]
#[pyo3(signature = (value, dehydration_hooks=None))]
fn pack<'py>(
    value: &Bound<'py, PyAny>,
    dehydration_hooks: Option<&Bound<'py, PyAny>>,
) -> PyResult<Bound<'py, PyBytes>> {
    super::v1::pack::pack::<extension::PackStreamV2Ext>(value, dehydration_hooks)
}

#[pyfunction]
#[pyo3(signature = (bytes, idx, hydration_hooks=None))]
fn unpack(
    bytes: Bound<PyByteArray>,
    idx: usize,
    hydration_hooks: Option<Bound<PyDict>>,
) -> PyResult<(Py<PyAny>, usize)> {
    super::v1::unpack::unpack::<extension::PackStreamV2Ext>(bytes, idx, hydration_hooks)
}

pub(crate) fn init_module(m: &Bound<PyModule>, name: &str) -> PyResult<()> {
    register_package(m, name)?;

    m.add_function(wrap_pyfunction!(unpack, m)?)?;
    m.add_function(wrap_pyfunction!(pack, m)?)?;

    Ok(())
}
