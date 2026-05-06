from __future__ import annotations
from typing import Protocol

class Tensor:
    data: list[float]
    def add(self, other: Tensor) -> Tensor: ...

