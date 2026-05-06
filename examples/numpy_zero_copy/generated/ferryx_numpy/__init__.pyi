from __future__ import annotations
from typing import Protocol

class FloatBuffer:
    data: list[float]
    def as_slice_len(self) -> int: ...

