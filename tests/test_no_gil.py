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


from __future__ import annotations

import importlib
import multiprocessing as mp
import sys
import sysconfig

import pytest


IS_NO_GIL_BUILD = sysconfig.get_config_var("Py_GIL_DISABLED") == 1


@pytest.mark.skipif(
    not IS_NO_GIL_BUILD,
    reason="This test only applies to no-GIL builds",
)
@pytest.mark.parametrize(
    "name",
    (
        "neo4j._rust.codec.packstream.v1",
        "neo4j._rust.codec.packstream",
        "neo4j._rust.codec",
        "neo4j._rust.vector",
        "neo4j._rust",
    ),
)
def test_gil_remains_disabled(name: str) -> None:
    # Running the test in a multiprocessing process to ensure we observe the
    # GIL unaffected by any packages already loaded (such as 3rd party
    # dependencies)
    p = mp.Process(target=_assert_gil_disabled_after_import, args=(name,))
    p.start()
    p.join()
    assert p.exitcode == 0


def _assert_gil_disabled_after_import(name: str):
    # WHEN: module is loaded
    _ = importlib.import_module(name)

    # THEN: GIL is still disabled
    assert _gil_is_disabled()


def _gil_is_disabled() -> bool:
    if not hasattr(sys, "_is_gil_enabled"):
        return False
    return not sys._is_gil_enabled()
