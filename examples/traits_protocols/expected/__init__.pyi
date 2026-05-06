from typing import Protocol

class ScorableProtocol(Protocol):
    def score(self) -> float: ...

class ModelOutput:
    confidence: float
    def calibrated(self, factor: float) -> "ModelOutput": ...

