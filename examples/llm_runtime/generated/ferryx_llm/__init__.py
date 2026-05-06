from __future__ import annotations

class LlmRuntime:
    model: str
    def __repr__(self) -> str:
        return "LlmRuntime(...)"
    def _repr_markdown_(self) -> str:
        return self.__repr__()
    def complete(self, prompt: str) -> str:
        raise NotImplementedError("Bound at runtime")

