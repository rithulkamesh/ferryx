# OpenAPI and gRPC Projection

ferryx projects service semantics from IR into:

- OpenAPI (`openapi.json`)
- protobuf service definitions (`.proto`)

## Mapping Rules

- impl methods -> operations/rpcs
- Rust scalar and collection types -> schema/proto field mappings
- async methods -> asynchronous endpoint semantics

## Validation

Cross-target parity checks live in `verification/cross_target/verify_cross_target.py`.

