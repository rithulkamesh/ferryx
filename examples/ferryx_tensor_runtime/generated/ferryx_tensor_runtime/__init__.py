from __future__ import annotations

class TensorRuntime:
    shape: list[int]
    data: list[float]
    def __repr__(self) -> str:
        return "TensorRuntime(...)"
    def _repr_markdown_(self) -> str:
        return self.__repr__()
    def new(self, shape: list[int], data: list[float]) -> TensorRuntime:
        raise NotImplementedError("Bound at runtime")
    def matmul2x2(self, other: TensorRuntime) -> TensorRuntime:
        raise NotImplementedError("Bound at runtime")
    def scale_async(self, factor: float) -> TensorRuntime:
        raise NotImplementedError("Bound at runtime")


class StringError(Exception):
    pass

