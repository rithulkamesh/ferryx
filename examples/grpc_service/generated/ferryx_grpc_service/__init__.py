from __future__ import annotations

class EchoService:
    pass
    def __repr__(self) -> str:
        return "EchoService(...)"
    def _repr_markdown_(self) -> str:
        return self.__repr__()
    def echo(self, message: str) -> str:
        raise NotImplementedError("Bound at runtime")


class StringError(Exception):
    pass

