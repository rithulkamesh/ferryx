from __future__ import annotations
from typing import Protocol

class FastTensor:
    data: list[float]
    def dot(self, other: FastTensor) -> float: ...

