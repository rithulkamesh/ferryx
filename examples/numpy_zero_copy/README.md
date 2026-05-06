# numpy_zero_copy example

Demonstrates data-shape contracts for zero-copy friendly buffer projection.

Current implementation models contiguous float buffers via `Vec<f32>`.
Future emitter/runtime layers can map this to NumPy views with explicit lifetime guarantees.

