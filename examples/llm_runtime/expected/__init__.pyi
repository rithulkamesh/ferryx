class LlmRuntime:
    model: str
    async def complete(self, prompt: str) -> str: ...

