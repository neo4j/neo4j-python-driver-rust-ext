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

import pytest

from ..vector.from_driver.test_vector import (
    _mock_mask_extensions,
    _swap_endian,
)


@pytest.mark.parametrize("ext", ("numpy", "rust", "python"))
@pytest.mark.parametrize("type_size", (2, 4, 8))
@pytest.mark.parametrize("length", (1, 100_000))
def test_bench_swap_endian(benchmark, mocker, ext, type_size, length):
    data = bytes(i % 256 for i in range(8 * length))
    _mock_mask_extensions(mocker, ext)
    rounds = max(min(1_000_000 // length, 100_000), 100)

    benchmark.pedantic(lambda: _swap_endian(type_size, data), rounds=rounds)
