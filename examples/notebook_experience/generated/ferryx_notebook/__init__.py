from __future__ import annotations

class TensorPreview:
    shape: list[int]
    sample: list[float]
    def __repr__(self) -> str:
        return "TensorPreview(...)"
    def _repr_markdown_(self) -> str:
        return self.__repr__()
    def summary(self) -> str:
        raise NotImplementedError("Bound at runtime")

