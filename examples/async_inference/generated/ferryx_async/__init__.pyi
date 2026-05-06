from __future__ import annotations
from typing import Protocol

class AsyncModel:
    name: str
    def infer(self, input: list[float]) -> list[float]: ...

