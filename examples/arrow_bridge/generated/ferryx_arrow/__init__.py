from __future__ import annotations

class ArrowColumn:
    name: str
    values: list[int]
    def __repr__(self) -> str:
        return "ArrowColumn(...)"
    def _repr_markdown_(self) -> str:
        return self.__repr__()
    def len(self) -> int:
        raise NotImplementedError("Bound at runtime")

