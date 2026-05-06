from __future__ import annotations

class NotebookKernel:
    session_id: str
    def __repr__(self) -> str:
        return "NotebookKernel(...)"
    def _repr_markdown_(self) -> str:
        return self.__repr__()
    def execute_cell(self, code: str) -> str:
        raise NotImplementedError("Bound at runtime")

