# Distribution and Packaging

## Wheel Strategy

- manylinux wheels via maturin + auditwheel
- macOS universal targets (`x86_64`, `aarch64`)
- Windows wheels for `x86_64`

## Python Support Matrix

- Python 3.9 through 3.13

## Rust Toolchain Matrix

- stable (required)
- beta (compatibility signal)
- nightly (forward-compatibility signal)

## Release Pipeline

1. run CI matrix (Rust + Python)
2. run benchmark gate jobs
3. build wheels
4. publish artifacts
5. tag release and changelog

