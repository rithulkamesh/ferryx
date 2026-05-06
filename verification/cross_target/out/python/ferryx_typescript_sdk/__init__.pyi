from __future__ import annotations
from typing import Protocol

class UserApi:
    service_name: str
    def get_user(self, id: str) -> str: ...


class StringError(Exception): ...
