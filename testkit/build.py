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


from _common import (
    run_python,
    TEST_LOCAL_DRIVER,
)


if __name__ == "__main__":
    run_python(["-m", "pip", "install", "-U", "pip"])
    run_python(["-m", "pip", "install", "-U", "--group", "packaging"])
    run_python(["-m", "build", "."])
    run_python(
        [
            "-m",
            "pip",
            "install",
            "-U",
            "--group",
            "driver/pyproject.toml:testkit",
            "-e",
            ".",
        ]
    )
    if TEST_LOCAL_DRIVER:
        run_python(["-m", "pip", "install", "./driver", "--no-deps"])
