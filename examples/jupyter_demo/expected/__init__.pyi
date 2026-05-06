class NotebookKernel:
    session_id: str
    def execute_cell(self, code: str) -> str: ...

