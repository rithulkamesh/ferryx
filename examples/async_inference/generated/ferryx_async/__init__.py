from __future__ import annotations

class AsyncModel:
    name: str
    def __repr__(self) -> str:
        return "AsyncModel(...)"
    def _repr_markdown_(self) -> str:
        return self.__repr__()
    def infer(self, input: list[float]) -> list[float]:
        raise NotImplementedError("Bound at runtime")

