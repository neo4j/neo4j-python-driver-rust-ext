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
from functools import cache
from pathlib import Path


__all__ = ["load_driver_test_module"]


@cache
def load_driver_test_module(mod: str):
    path = Path(__file__).parents[1] / "driver" / "tests"
    parts = mod.split(".") if mod else ()
    for part in parts:
        if not part:
            raise ValueError(f"Invalid module name: {mod!r}")
        if part == "__init__":
            raise ValueError(f"Invalid module name: {mod!r}")
        path /= part
    path = path.resolve().absolute()
    if path.is_dir():
        path /= "__init__"
    path = path.with_suffix(".py")
    name = ".".join(("driver_tests", *parts))
    spec = importlib.util.spec_from_file_location(name, str(path))
    if spec is None:
        raise ImportError(f"Could not load module {mod} from {path}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def _inject_root_mod():
    import sys

    mod = load_driver_test_module("")
    sys.modules[mod.__name__] = mod


_inject_root_mod()
del _inject_root_mod
