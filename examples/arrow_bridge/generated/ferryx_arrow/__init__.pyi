from __future__ import annotations
from typing import Protocol

class ArrowColumn:
    name: str
    values: list[int]
    def len(self) -> int: ...

