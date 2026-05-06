from __future__ import annotations

class Tensor:
    data: list[float]
    def __repr__(self) -> str:
        return "Tensor(...)"
    def _repr_markdown_(self) -> str:
        return self.__repr__()
    def add(self, other: Tensor) -> Tensor:
        raise NotImplementedError("Bound at runtime")

