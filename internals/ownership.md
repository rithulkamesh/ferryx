# Internals: Ownership Model

Ownership is carried from Rust signatures into IR:

- `Owned`
- `Borrowed { mutable, lifetime }`

Emitters use ownership metadata to choose API surfaces and future runtime borrowing strategies.

Current Python projection is value-centric; ownership metadata remains available for advanced zero-copy and async safety features.

