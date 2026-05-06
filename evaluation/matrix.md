# Benchmark Comparison Matrix

This matrix defines benchmark suites and adapter targets.

| Suite | ferryx | PyO3 | cffi | ctypes | pybind11 |
|---|---|---|---|---|---|
| call overhead | benchmark harness | adapter script | adapter script | adapter script | adapter script |
| zero-copy throughput | benchmark harness | adapter script | adapter script | adapter script | adapter script |
| NumPy interop | benchmark harness | adapter script | adapter script | adapter script | adapter script |
| async latency | benchmark harness | adapter script | adapter script | adapter script | adapter script |
| serialization throughput | benchmark harness | adapter script | adapter script | adapter script | adapter script |
| memory overhead | benchmark harness | adapter script | adapter script | adapter script | adapter script |
| wheel import time | benchmark harness | adapter script | adapter script | adapter script | adapter script |
| API generation speed | benchmark harness | n/a | n/a | n/a | n/a |

Adapter scripts should emit normalized JSON records into `evaluation/results/`.

