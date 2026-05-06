from __future__ import annotations

class FastTensor:
    data: list[float]
    def __repr__(self) -> str:
        return "FastTensor(...)"
    def _repr_markdown_(self) -> str:
        return self.__repr__()
    def dot(self, other: FastTensor) -> float:
        raise NotImplementedError("Bound at runtime")

