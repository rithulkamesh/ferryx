from __future__ import annotations

class UserProfile:
    id: int
    name: str
    active: bool
    def __repr__(self) -> str:
        return "UserProfile(...)"
    def _repr_markdown_(self) -> str:
        return self.__repr__()
    def rename(self, new_name: str) -> UserProfile:
        raise NotImplementedError("Bound at runtime")

