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

use std::ffi::CStr;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use super::super::uuid::get_uuid_cls;
use super::pack::PackStreamEncoder;
use super::unpack::PackStreamDecoder;

pub(crate) trait PackStreamV1Ext: Sized {
    fn type_mapping_import() -> &'static CStr;
    fn pack_ext(
        value: &'_ Bound<PyAny>,
        encoder: &mut PackStreamEncoder<'_, Self>,
    ) -> PyResult<bool>;
    fn unpack_ext(marker: u8, decoder: &mut PackStreamDecoder<Self>)
        -> PyResult<Option<Py<PyAny>>>;
}

pub(crate) struct PackStreamV1BaseExt {}

impl PackStreamV1Ext for PackStreamV1BaseExt {
    #[inline]
    fn type_mapping_import() -> &'static CStr {
        c"from neo4j._codec.packstream.v1.types import *"
    }

    #[inline]
    fn pack_ext(value: &'_ Bound<PyAny>, _: &mut PackStreamEncoder<'_, Self>) -> PyResult<bool> {
        let py = value.py();

        let uuid_cls = get_uuid_cls(py)?;
        if value.is_instance(uuid_cls)? {
            return Err(PyErr::new::<PyValueError, _>(format!(
                "Values of type {} are not supported \
                (requires Bolt protocol version 6.1 or newer)",
                value.get_type().str()?
            )));
        }

        Ok(false)
    }

    #[inline]
    fn unpack_ext(_: u8, _: &mut PackStreamDecoder<Self>) -> PyResult<Option<Py<PyAny>>> {
        Ok(None)
    }
}
