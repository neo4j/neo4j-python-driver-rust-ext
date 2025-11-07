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


import os
import subprocess
import sys


_TRUE_ENV_VALS = {"1", "y", "yes", "true", "t", "on"}


TEST_BACKEND_VERSION = os.getenv("TEST_BACKEND_VERSION", "python")
TEST_LOCAL_DRIVER = (
    os.environ.get("TEST_LOCAL_DRIVER", "").lower() in _TRUE_ENV_VALS
)


def run(args, env=None, **kwargs):
    print(args)
    return subprocess.run(
        args,
        text=True,
        stdout=sys.stdout,
        stderr=sys.stderr,
        check=True,
        env=env,
        **kwargs,
    )


def run_python(args, env=None, **kwargs):
    cmd = [TEST_BACKEND_VERSION, "-u", *args]
    run(cmd, env=env, **kwargs)
