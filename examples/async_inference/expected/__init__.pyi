class AsyncModel:
    name: str
    async def infer(self, input: list[float]) -> list[float]: ...

