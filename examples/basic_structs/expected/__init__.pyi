class UserProfile:
    id: int
    name: str
    active: bool
    def rename(self, new_name: str) -> "UserProfile": ...

