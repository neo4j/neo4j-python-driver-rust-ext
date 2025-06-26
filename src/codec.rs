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

mod packstream;

use pyo3::prelude::*;

use crate::register_package;

pub(super) fn init_module(m: &Bound<PyModule>, name: &str) -> PyResult<()> {
    let py = m.py();

    m.gil_used(false)?;
    register_package(m, name)?;

    let mod_packstream = PyModule::new(py, "packstream")?;
    m.add_submodule(&mod_packstream)?;
    packstream::init_module(&mod_packstream, format!("{name}.packstream").as_str())?;

    Ok(())
}
