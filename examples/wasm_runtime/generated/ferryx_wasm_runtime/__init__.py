from __future__ import annotations

class WasmMath:
    scale: float
    def __repr__(self) -> str:
        return "WasmMath(...)"
    def _repr_markdown_(self) -> str:
        return self.__repr__()
    def mul(self, v: list[float]) -> list[float]:
        raise NotImplementedError("Bound at runtime")

