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
import sys
import traceback

import pytest

from neo4j._codec.hydration import DehydrationHooks
from neo4j._codec.packstream import Structure
from neo4j._codec.packstream.v1 import (
    Packer,
    Unpacker,
)


@pytest.fixture
def packer_with_buffer():
    packable_buffer = Packer.new_packable_buffer()
    return Packer(packable_buffer), packable_buffer


@pytest.fixture
def unpacker_with_buffer():
    unpackable_buffer = Unpacker.new_unpackable_buffer()
    return Unpacker(unpackable_buffer), unpackable_buffer


def test_pack_injection_works(packer_with_buffer):
    class TestClass:
        pass

    class TestError(Exception):
        pass

    def raise_test_exception(*args, **kwargs):
        raise TestError

    dehydration_hooks = DehydrationHooks(
        exact_types={TestClass: raise_test_exception},
        subtypes={},
    )
    test_object = TestClass()
    packer, _ = packer_with_buffer

    with pytest.raises(TestError) as exc:
        packer.pack(test_object, dehydration_hooks=dehydration_hooks)

    # printing the traceback to stdout to make it easier to debug
    traceback.print_exception(exc.type, exc.value, exc.tb, file=sys.stdout)

    assert any("_rust_pack" in str(entry.statement) for entry in exc.traceback)
    assert not any(
        "_py_pack" in str(entry.statement) for entry in exc.traceback
    )


def test_unpack_injection_works(unpacker_with_buffer):
    class TestError(Exception):
        pass

    def raise_test_exception(*args, **kwargs):
        raise TestError

    hydration_hooks = {Structure: raise_test_exception}
    unpacker, buffer = unpacker_with_buffer

    buffer.reset()
    buffer.data = bytearray(b"\xb0\xff")

    with pytest.raises(TestError) as exc:
        unpacker.unpack(hydration_hooks)

    # printing the traceback to stdout to make it easier to debug
    traceback.print_exception(exc.type, exc.value, exc.tb, file=sys.stdout)

    assert any(
        "_rust_unpack" in str(entry.statement) for entry in exc.traceback
    )
    assert not any(
        "_py_unpack" in str(entry.statement) for entry in exc.traceback
    )


@pytest.mark.parametrize(
    ("name", "package_names"),
    (
        ("neo4j._codec.packstream._rust.v1", ()),
        ("neo4j._codec.packstream._rust", ("v1",)),
        ("neo4j._codec.packstream", ("_rust",)),
    ),
)
def test_import_module(name, package_names):
    module = importlib.import_module(name)

    assert module.__name__ == name

    for package_name in package_names:
        package = getattr(module, package_name)
        assert package.__name__ == f"{name}.{package_name}"


def test_rust_struct_access():
    tag = b"F"
    fields = ["foo", False, 42, 3.14, b"bar"]
    struct = Structure(tag, *fields)

    assert struct.tag == tag
    assert isinstance(struct.tag, bytes)
    assert struct.fields == tuple(fields)


def test_rust_struct_equal():
    struct1 = Structure(b"F", "foo", False, 42, 3.14, b"bar")
    struct2 = Structure(b"F", "foo", False, 42, 3.14, b"bar")
    assert struct1 == struct2
    # [noqa] for testing correctness of equality
    assert not struct1 != struct2  # noqa: SIM202


@pytest.mark.parametrize(
    "args",
    (
        (b"F", "foo", True, 42, 3.14, b"bar"),
        (b"f", "foo", False, 42, 3.14, b"baz"),
    ),
)
def test_rust_struct_not_equal(args):
    struct1 = Structure(b"F", "foo", False, 42, 3.14, b"bar")
    struct2 = Structure(*args)
    assert struct1 != struct2
    # [noqa] for testing correctness of equality
    assert not struct1 == struct2  # noqa: SIM201
