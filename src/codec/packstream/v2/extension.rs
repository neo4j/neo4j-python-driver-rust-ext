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

use pyo3::intern;
use pyo3::prelude::*;
use pyo3::IntoPyObjectExt;

use super::super::uuid::get_uuid_cls;
use super::super::v1::extension::PackStreamV1Ext;
use super::super::v1::pack::PackStreamEncoder;
use super::super::v1::unpack::PackStreamDecoder;
use super::UUID;

pub(crate) struct PackStreamV2Ext {}

impl PackStreamV2Ext {
    fn write_uuid(uuid: u128, encoder: &mut PackStreamEncoder<'_, Self>) {
        encoder.write_raw(&[UUID]);
        encoder.write_raw(&u128::to_be_bytes(uuid));
    }
}

impl PackStreamV1Ext for PackStreamV2Ext {
    #[inline]
    fn type_mapping_import() -> &'static CStr {
        c"from neo4j._codec.packstream.v2.types import *"
    }

    #[inline]
    fn pack_ext(
        value: &'_ Bound<PyAny>,
        encoder: &mut PackStreamEncoder<'_, Self>,
    ) -> PyResult<bool> {
        let py = value.py();

        let uuid_cls = get_uuid_cls(py)?;
        if value.is_instance(uuid_cls)? {
            let uuid_int: u128 = value.getattr(intern!(py, "int"))?.extract()?;
            Self::write_uuid(uuid_int, encoder);
            return Ok(true);
        }

        Ok(false)
    }

    #[inline]
    fn unpack_ext(
        marker: u8,
        decoder: &mut PackStreamDecoder<Self>,
    ) -> PyResult<Option<Py<PyAny>>> {
        let py = decoder.py();

        Ok(match marker {
            UUID => {
                let uuid_cls = get_uuid_cls(py)?;
                let uuid_int = u128::from_be_bytes(decoder.read_n_bytes()?);
                let uuid_obj =
                    uuid_cls.call1((py.None(), py.None(), py.None(), py.None(), uuid_int))?;
                Some(uuid_obj.into_py_any(py)?)
            }
            _ => None,
        })
    }
}
