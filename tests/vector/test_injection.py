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

import importlib

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
    ("name", "submodule_names"),
    (
        ("neo4j._rust.vector", ()),
        ("neo4j._rust", ("vector",)),
        ("neo4j", ("_rust",)),
    ),
)
def test_import_module(name, submodule_names):
    module = importlib.import_module(name)

    assert module.__name__ == name

    for submodule_name in submodule_names:
        package = getattr(module, submodule_name)
        assert package.__name__ == f"{name}.{submodule_name}"
