from __future__ import annotations

class FloatBuffer:
    data: list[float]
    def __repr__(self) -> str:
        return "FloatBuffer(...)"
    def _repr_markdown_(self) -> str:
        return self.__repr__()
    def as_slice_len(self) -> int:
        raise NotImplementedError("Bound at runtime")

