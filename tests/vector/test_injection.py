# Copyright (c) "Neo4j"
# Neo4j Sweden AB [https://neo4j.com]
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     https://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.


import pytest

import neo4j.vector


def test_endian_swap_was_imported():
    swap = neo4j.vector._swap_endian_unchecked_rust
    assert swap is not None
    assert swap is neo4j._rust.vector.swap_endian
    assert neo4j.vector._swap_endian_unchecked is swap


def test_endian_swap_was_injected(mocker):
    mock = mocker.patch("neo4j.vector._swap_endian_unchecked")
    neo4j.vector._swap_endian(2, b"\x01\x02\x03\x04")
    mock.assert_called_once_with(2, b"\x01\x02\x03\x04")


@pytest.mark.parametrize(
    "vec_cls",
    (
        neo4j.vector._VecF64,
        neo4j.vector._VecF32,
        neo4j.vector._VecI64,
        neo4j.vector._VecI32,
        neo4j.vector._VecI16,
        neo4j.vector._VecI8,
    ),
)
def test_vec_from_native_was_imported(vec_cls):
    vec_rust = neo4j.vector._vec_rust
    assert vec_rust is not None
    assert vec_cls.from_native == vec_cls._from_native_rust


@pytest.mark.parametrize(
    ("dtype", "value", "method"),
    (
        ("f64", 1.0, "vec_f64_from_native"),
        ("f32", 1.0, "vec_f32_from_native"),
        ("i64", 1, "vec_i64_from_native"),
        ("i32", 1, "vec_i32_from_native"),
        ("i16", 1, "vec_i16_from_native"),
        ("i8", 1, "vec_i8_from_native"),
    ),
)
def test_vec_from_native_was_injected(dtype, value, method, mocker):
    mock = mocker.patch("neo4j.vector._vec_rust")
    rust_mock = getattr(mock, method)
    rust_mock.return_value = b""

    data = [value]

    neo4j.vector.Vector.from_native(data, dtype)

    getattr(mock, method).assert_called_once_with(data)


@pytest.mark.parametrize(
    "vec_cls",
    (
        neo4j.vector._VecF64,
        neo4j.vector._VecF32,
        neo4j.vector._VecI64,
        neo4j.vector._VecI32,
        neo4j.vector._VecI16,
        neo4j.vector._VecI8,
    ),
)
def test_vec_to_native_was_imported(vec_cls):
    vec_rust = neo4j.vector._vec_rust
    assert vec_rust is not None
    assert vec_cls.to_native == vec_cls._to_native_rust


@pytest.mark.parametrize(
    ("dtype", "method"),
    (
        ("f64", "vec_f64_to_native"),
        ("f32", "vec_f32_to_native"),
        ("i64", "vec_i64_to_native"),
        ("i32", "vec_i32_to_native"),
        ("i16", "vec_i16_to_native"),
        ("i8", "vec_i8_to_native"),
    ),
)
def test_vec_to_native_was_injected(dtype, method, mocker):
    mock = mocker.patch("neo4j.vector._vec_rust")

    data = bytes(range(8))
    vec = neo4j.vector.Vector.from_bytes(data, dtype)
    getattr(mock, method).assert_not_called()

    vec.to_native()

    getattr(mock, method).assert_called_once_with(data)
