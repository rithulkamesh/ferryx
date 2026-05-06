from __future__ import annotations

class UserApi:
    service_name: str
    def __repr__(self) -> str:
        return "UserApi(...)"
    def _repr_markdown_(self) -> str:
        return self.__repr__()
    def get_user(self, id: str) -> str:
        raise NotImplementedError("Bound at runtime")


class StringError(Exception):
    pass

