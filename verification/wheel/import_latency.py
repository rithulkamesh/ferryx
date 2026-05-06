#!/usr/bin/env python3
import time
import importlib


def main():
    start = time.perf_counter()
    importlib.import_module("json")
    elapsed_ms = (time.perf_counter() - start) * 1000
    print(f"import_latency_ms={elapsed_ms:.3f}")


if __name__ == "__main__":
    main()

