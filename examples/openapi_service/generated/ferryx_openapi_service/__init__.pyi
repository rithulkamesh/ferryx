from __future__ import annotations
from typing import Protocol

class HealthService:
    ...
    def check(self) -> str: ...

