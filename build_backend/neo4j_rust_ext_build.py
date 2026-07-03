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

"""PEP 517 build backend that wraps maturin.

The backend chooses a `Cargo.toml` manifest based on the Python version.
All other build backend functions are delegated to maturin.
"""

import os
import sys

import maturin


_PROJECT_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
_PY37_DIR = os.path.join(_PROJECT_ROOT, "cargo_py37")
_PY37_MANIFEST = os.path.join(_PY37_DIR, "Cargo.toml")


def _inject_legacy_manifest(config_settings):
    if sys.version_info >= (3, 8):
        return config_settings
    cs = dict(config_settings or {})
    extra = f"--manifest-path {_PY37_MANIFEST}"
    existing = cs.get("build-args", "")
    cs["build-args"] = f"{existing} {extra}".strip() if existing else extra
    return cs


def build_wheel(
    wheel_directory, config_settings=None, metadata_directory=None
):
    return maturin.build_wheel(
        wheel_directory,
        _inject_legacy_manifest(config_settings),
        metadata_directory,
    )


def build_editable(
    wheel_directory, config_settings=None, metadata_directory=None
):
    return maturin.build_editable(
        wheel_directory,
        _inject_legacy_manifest(config_settings),
        metadata_directory,
    )


get_requires_for_build_wheel = maturin.get_requires_for_build_wheel
get_requires_for_build_editable = maturin.get_requires_for_build_editable
get_requires_for_build_sdist = maturin.get_requires_for_build_sdist
prepare_metadata_for_build_wheel = maturin.prepare_metadata_for_build_wheel
prepare_metadata_for_build_editable = (
    maturin.prepare_metadata_for_build_editable
)
build_sdist = maturin.build_sdist
