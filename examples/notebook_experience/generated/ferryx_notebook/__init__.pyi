from __future__ import annotations
from typing import Protocol

class TensorPreview:
    shape: list[int]
    sample: list[float]
    def summary(self) -> str: ...

