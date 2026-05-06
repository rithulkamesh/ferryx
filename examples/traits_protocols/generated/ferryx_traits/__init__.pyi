from __future__ import annotations
from typing import Protocol

class ModelOutput:
    confidence: float
    def score(self) -> float: ...
    def calibrated(self, factor: float) -> ModelOutput: ...

