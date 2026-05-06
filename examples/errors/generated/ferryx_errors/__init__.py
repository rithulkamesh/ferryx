from __future__ import annotations

class Calculator:
    pass
    def __repr__(self) -> str:
        return "Calculator(...)"
    def _repr_markdown_(self) -> str:
        return self.__repr__()
    def divide(self, left: float, right: float) -> float:
        raise NotImplementedError("Bound at runtime")


class MathError(Exception):
    pass

