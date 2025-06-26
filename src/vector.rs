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

mod native_conversion;
mod swap_endian;

use crate::register_package;
use pyo3::prelude::*;

pub(super) fn init_module(m: &Bound<PyModule>, name: &str) -> PyResult<()> {
    m.gil_used(false)?;
    register_package(m, name)?;

    m.add_function(wrap_pyfunction!(swap_endian::swap_endian, m)?)?;
    m.add_function(wrap_pyfunction!(native_conversion::vec_f64_from_native, m)?)?;
    m.add_function(wrap_pyfunction!(native_conversion::vec_f64_to_native, m)?)?;
    m.add_function(wrap_pyfunction!(native_conversion::vec_f32_from_native, m)?)?;
    m.add_function(wrap_pyfunction!(native_conversion::vec_f32_to_native, m)?)?;
    m.add_function(wrap_pyfunction!(native_conversion::vec_i64_from_native, m)?)?;
    m.add_function(wrap_pyfunction!(native_conversion::vec_i64_to_native, m)?)?;
    m.add_function(wrap_pyfunction!(native_conversion::vec_i32_from_native, m)?)?;
    m.add_function(wrap_pyfunction!(native_conversion::vec_i32_to_native, m)?)?;
    m.add_function(wrap_pyfunction!(native_conversion::vec_i16_from_native, m)?)?;
    m.add_function(wrap_pyfunction!(native_conversion::vec_i16_to_native, m)?)?;
    m.add_function(wrap_pyfunction!(native_conversion::vec_i8_from_native, m)?)?;
    m.add_function(wrap_pyfunction!(native_conversion::vec_i8_to_native, m)?)?;

    Ok(())
}
