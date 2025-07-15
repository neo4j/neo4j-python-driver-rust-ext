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


from collections import defaultdict

import pytest


@pytest.mark.hookwrapper
def pytest_benchmark_group_stats(config, benchmarks, group_by):
    outcome = yield

    if group_by != "group":
        # not default grouping, so let the user have what they asked for
        return

    result = defaultdict(list)
    for bench in benchmarks:
        param_start = bench["fullname"].rfind("[")
        if param_start < 0:
            base_name = bench["fullname"]
        else:
            base_name = bench["fullname"][:param_start]
        params = bench["params"]
        ext = params.get("ext", None)
        if ext:
            param_keys = sorted(params.keys())
            name_params = "-".join(
                str(params[k]) for k in param_keys if k != "ext"
            )
            group_name = f"{base_name}[{name_params}]"
        else:
            group_name = base_name
        result[group_name].append(bench)

    outcome.force_result(result.items())
