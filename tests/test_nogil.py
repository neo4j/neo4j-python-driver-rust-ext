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

import importlib.util
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
    ("ext_module", "submodule"),
    (
        ("neo4j._codec.packstream._rust", "v1"),
        ("neo4j._codec.packstream._rust", ""),
    ),
)
def test_gil_remains_disabled(ext_module: str, submodule: str) -> None:
    spec = importlib.util.find_spec(ext_module)
    if spec is None or spec.origin is None or spec.loader is None:
        raise ImportError(f"Could not find spec for {ext_module}")

    # Running the test in a multiprocessing process to ensure we observe the
    # GIL unaffected by any packages already loaded (such as 3rd party
    # dependencies)
    p = mp.Process(
        target=_assert_gil_disabled_after_import,
        args=(ext_module, spec.origin, submodule),
    )
    p.start()
    p.join()
    assert p.exitcode == 0


def _assert_gil_disabled_after_import(
    mod_name: str, mod_path: str, submodule: str
) -> None:
    assert _gil_is_disabled()  # sanity check

    # WHEN: module is loaded
    # Manually loading the extension module from path to avoid loading the
    # driver which in turn might end up importing 3rd party libraries that
    # could enable the GIL while we only want to test our extension module.
    ext_spec = importlib.util.spec_from_file_location(mod_name, mod_path)
    if ext_spec is None or ext_spec.loader is None:
        raise ImportError(f"Could not find spec for {mod_name} at {mod_path}")
    loaded_module = importlib.util.module_from_spec(ext_spec)
    ext_spec.loader.exec_module(loaded_module)
    for attr_name in submodule.split(".") if submodule else ():
        loaded_module = getattr(loaded_module, attr_name)

    # THEN: GIL is still disabled
    assert _gil_is_disabled()


def _gil_is_disabled() -> bool:
    if not hasattr(sys, "_is_gil_enabled"):
        return False
    return not sys._is_gil_enabled()
