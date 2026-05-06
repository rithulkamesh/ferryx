from __future__ import annotations

class HealthService:
    pass
    def __repr__(self) -> str:
        return "HealthService(...)"
    def _repr_markdown_(self) -> str:
        return self.__repr__()
    def check(self) -> str:
        raise NotImplementedError("Bound at runtime")

