from __future__ import annotations
from typing import Protocol

class NotebookKernel:
    session_id: str
    def execute_cell(self, code: str) -> str: ...

