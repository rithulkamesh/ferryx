from __future__ import annotations
from typing import Protocol

class LlmRuntime:
    model: str
    def complete(self, prompt: str) -> str: ...

