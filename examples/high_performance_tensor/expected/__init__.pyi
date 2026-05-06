class FastTensor:
    data: list[float]
    def dot(self, other: "FastTensor") -> float: ...

