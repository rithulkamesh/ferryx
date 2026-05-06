from __future__ import annotations
from typing import Protocol

class WasmMath:
    scale: float
    def mul(self, v: list[float]) -> list[float]: ...

