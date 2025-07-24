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

mod codec;
mod vector;

use pyo3::prelude::*;

#[pymodule(gil_used = false)]
#[pyo3(name = "_rust")]
fn init_module(m: &Bound<PyModule>) -> PyResult<()> {
    let py = m.py();

    let mod_codec = PyModule::new(py, "codec")?;
    m.add_submodule(&mod_codec)?;
    codec::init_module(&mod_codec, "codec")?;

    let mod_vector = PyModule::new(py, "vector")?;
    m.add_submodule(&mod_vector)?;
    vector::init_module(&mod_vector, "vector")?;

    Ok(())
}

// hack to make python pick up the submodule as a package
// https://github.com/PyO3/pyo3/issues/1517#issuecomment-808664021
fn register_package(m: &Bound<PyModule>, name: &str) -> PyResult<()> {
    let py = m.py();
    let module_name = format!("neo4j._rust.{name}").into_pyobject(py)?;

    py.import("sys")?
        .getattr("modules")?
        .set_item(&module_name, m)?;
    m.setattr("__name__", &module_name)?;

    Ok(())
}
