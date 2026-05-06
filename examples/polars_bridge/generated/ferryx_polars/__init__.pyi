from __future__ import annotations
from typing import Protocol

class ColumnBatch:
    name: str
    values: list[float]
    def mean(self) -> float: ...

