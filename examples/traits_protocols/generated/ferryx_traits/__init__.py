from __future__ import annotations

class ModelOutput:
    confidence: float
    def __repr__(self) -> str:
        return "ModelOutput(...)"
    def _repr_markdown_(self) -> str:
        return self.__repr__()
    def score(self) -> float:
        raise NotImplementedError("Bound at runtime")
    def calibrated(self, factor: float) -> ModelOutput:
        raise NotImplementedError("Bound at runtime")

