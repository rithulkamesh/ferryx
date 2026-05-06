from __future__ import annotations

class ColumnBatch:
    name: str
    values: list[float]
    def __repr__(self) -> str:
        return "ColumnBatch(...)"
    def _repr_markdown_(self) -> str:
        return self.__repr__()
    def mean(self) -> float:
        raise NotImplementedError("Bound at runtime")

