class ColumnBatch:
    name: str
    values: list[float]
    def mean(self) -> float: ...

