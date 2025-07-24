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


import gc
from contextlib import contextmanager

from neo4j._codec.packstream import Structure


@contextmanager
def gc_disabled():
    try:
        gc.disable()
        yield
    finally:
        gc.enable()
        gc.collect()


class StructureHolder:
    s: Structure | None = None


def test_memory_leak() -> None:
    iterations = 10_000

    gc.collect()
    with gc_disabled():
        for _ in range(iterations):
            # create a reference cycle
            holder1 = StructureHolder()
            structure1 = Structure(b"\x00", [holder1])
            holder2 = StructureHolder()
            structure2 = Structure(b"\x01", [holder2])
            holder1.s = structure2
            holder2.s = structure1
        del structure1, structure2, holder1, holder2

        cleaned = gc.collect()
        assert cleaned >= 4 * iterations
