# RTX4080
```bash
=== Platform: NVIDIA CUDA ===
Device: NVIDIA GeForce RTX 4080
  Type: GPU
  Version: 3.0
  cl_ext_float_atomics: false


=== Warming up (3 iterations) ===
Warmup 1: 1057 output points
Warmup 2: 1057 output points
Warmup 3: 1057 output points

=== Benchmarking (10 iterations) ===
Iteration 1: 0.051 ms
Iteration 2: 0.053 ms
Iteration 3: 0.050 ms
Iteration 4: 0.056 ms
Iteration 5: 0.052 ms
Iteration 6: 0.050 ms
Iteration 7: 0.050 ms
Iteration 8: 0.050 ms
Iteration 9: 0.049 ms
Iteration 10: 0.049 ms

=== Benchmark Results ===
Input points:  19968
Output points: 1057
Voxel size:    0.5

Timing statistics (ms):
  Mean:   0.051
  Median: 0.050
  Min:    0.049
  Max:    0.056
  Stddev: 0.002
```

```bash
=== Platform: NVIDIA CUDA ===
Device: NVIDIA GeForce RTX 4080
  Type: GPU
  Version: 3.0
  cl_ext_float_atomics: false


=== Warming up (3 iterations) ===
Warmup 1: 153903 output points
Warmup 2: 153903 output points
Warmup 3: 153903 output points

=== Benchmarking (10 iterations) ===
Iteration 1: 1.315 ms
Iteration 2: 1.306 ms
Iteration 3: 1.323 ms
Iteration 4: 1.324 ms
Iteration 5: 1.305 ms
Iteration 6: 1.315 ms
Iteration 7: 1.317 ms
Iteration 8: 1.305 ms
Iteration 9: 1.311 ms
Iteration 10: 1.325 ms

=== Benchmark Results ===
Input points:  1413157
Output points: 153903
Voxel size:    0.5

Timing statistics (ms):
  Mean:   1.315
  Median: 1.315
  Min:    1.305
  Max:    1.325
  Stddev: 0.007
```

# RTX3050
```bash
=== Platform: NVIDIA CUDA ===
Device: NVIDIA GeForce RTX 3050
  Type: GPU
  Version: 3.0
  cl_ext_float_atomics: false


=== Warming up (3 iterations) ===
Warmup 1: 1057 output points
Warmup 2: 1057 output points
Warmup 3: 1057 output points

=== Benchmarking (10 iterations) ===
Iteration 1: 0.096 ms
Iteration 2: 0.091 ms
Iteration 3: 0.088 ms
Iteration 4: 0.085 ms
Iteration 5: 0.087 ms
Iteration 6: 0.087 ms
Iteration 7: 0.086 ms
Iteration 8: 0.087 ms
Iteration 9: 0.087 ms
Iteration 10: 0.085 ms

=== Benchmark Results ===
Input points:  19968
Output points: 1057
Voxel size:    0.5

Timing statistics (ms):
  Mean:   0.088
  Median: 0.087
  Min:    0.085
  Max:    0.096
  Stddev: 0.003
```

```bash
=== Platform: NVIDIA CUDA ===
Device: NVIDIA GeForce RTX 3050
  Type: GPU
  Version: 3.0
  cl_ext_float_atomics: false


=== Warming up (3 iterations) ===
Warmup 1: 153903 output points
Warmup 2: 153903 output points
Warmup 3: 153903 output points

=== Benchmarking (10 iterations) ===
Iteration 1: 8.425 ms
Iteration 2: 8.450 ms
Iteration 3: 8.430 ms
Iteration 4: 8.445 ms
Iteration 5: 8.490 ms
Iteration 6: 8.479 ms
Iteration 7: 8.487 ms
Iteration 8: 8.459 ms
Iteration 9: 8.502 ms
Iteration 10: 8.450 ms

=== Benchmark Results ===
Input points:  1413157
Output points: 153903
Voxel size:    0.5

Timing statistics (ms):
  Mean:   8.462
  Median: 8.459
  Min:    8.425
  Max:    8.502
  Stddev: 0.025
```

# Intel Xe GPU (Windows11)
```bash
=== Platform: Intel(R) OpenCL Graphics ===
Device: Intel(R) Iris(R) Xe Graphics
  Type: GPU
  Version: 3.0
  cl_ext_float_atomics: true
  related: ["cl_ext_float_atomics"]

Using OpenCL device: Intel(R) Corporation / Intel(R) Iris(R) Xe Graphics

=== Warming up (3 iterations) ===
Warmup 1: 153903 output points
Warmup 2: 153903 output points
Warmup 3: 153903 output points

=== Benchmarking (10 iterations) ===
Iteration 1: 118.045 ms
Iteration 2: 119.147 ms
Iteration 3: 118.338 ms
Iteration 4: 118.203 ms
Iteration 5: 118.306 ms
Iteration 6: 117.806 ms
Iteration 7: 117.593 ms
Iteration 8: 117.734 ms
Iteration 9: 118.553 ms
Iteration 10: 117.958 ms

=== Benchmark Results ===
Input points:  1413157
Output points: 153903
Voxel size:    0.5

Timing statistics (ms):
  Mean:   118.168
  Median: 118.203
  Min:    117.593
  Max:    119.147
  Stddev: 0.432
```

Saving final global map with 668802 
Final global map points after voxel downsampling: 671522