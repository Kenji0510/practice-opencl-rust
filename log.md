# RTX4080
```bash
=== Platform: NVIDIA CUDA ===
Device: NVIDIA GeForce RTX 4080
  Type: GPU
  Version: 3.0
  cl_ext_float_atomics: false

=== Initializing GPU context ===
Using Platform: NVIDIA CUDA
Using Device:   NVIDIA GeForce RTX 4080
=== Completed GPU context ===

=== Warming up (3 iterations) ===
Warmup 1: 1057 output points
Warmup 2: 1057 output points
Warmup 3: 1057 output points

=== Benchmarking (10 iterations) ===
Iteration 1: Voxelization: 0.058 ms
Iteration 1: Covariance computation: 0.269 ms
Iteration 1: 0.269 ms
Iteration 2: Voxelization: 0.051 ms
Iteration 2: Covariance computation: 0.262 ms
Iteration 2: 0.262 ms
Iteration 3: Voxelization: 0.060 ms
Iteration 3: Covariance computation: 0.268 ms
Iteration 3: 0.268 ms
Iteration 4: Voxelization: 0.052 ms
Iteration 4: Covariance computation: 0.264 ms
Iteration 4: 0.264 ms
Iteration 5: Voxelization: 0.051 ms
Iteration 5: Covariance computation: 0.262 ms
Iteration 5: 0.262 ms
Iteration 6: Voxelization: 0.052 ms
Iteration 6: Covariance computation: 0.263 ms
Iteration 6: 0.263 ms
Iteration 7: Voxelization: 0.052 ms
Iteration 7: Covariance computation: 0.264 ms
Iteration 7: 0.264 ms
Iteration 8: Voxelization: 0.053 ms
Iteration 8: Covariance computation: 0.262 ms
Iteration 8: 0.262 ms
Iteration 9: Voxelization: 0.051 ms
Iteration 9: Covariance computation: 0.261 ms
Iteration 9: 0.261 ms
Iteration 10: Voxelization: 0.062 ms
Iteration 10: Covariance computation: 0.271 ms
Iteration 10: 0.271 ms

=== Benchmark Results ===
Input points:  19968
Output points: 1057
Voxel size:    0.5

Timing statistics (ms):
  Mean:   0.265
  Median: 0.264
  Min:    0.261
  Max:    0.271
  Stddev: 0.003
```

```bash
=== Platform: NVIDIA CUDA ===
Device: NVIDIA GeForce RTX 4080
  Type: GPU
  Version: 3.0
  cl_ext_float_atomics: false

=== Initializing GPU context ===
Using Platform: NVIDIA CUDA
Using Device:   NVIDIA GeForce RTX 4080
=== Completed GPU context ===

=== Warming up (3 iterations) ===
Warmup 1: 153903 output points
Warmup 2: 153903 output points
Warmup 3: 153903 output points

=== Benchmarking (10 iterations) ===
Iteration 1: Voxelization: 1.835 ms
Iteration 1: Covariance computation: 46.150 ms
Iteration 1: 46.150 ms
Iteration 2: Voxelization: 1.544 ms
Iteration 2: Covariance computation: 43.026 ms
Iteration 2: 43.026 ms
Iteration 3: Voxelization: 1.523 ms
Iteration 3: Covariance computation: 42.848 ms
Iteration 3: 42.848 ms
Iteration 4: Voxelization: 1.561 ms
Iteration 4: Covariance computation: 42.619 ms
Iteration 4: 42.619 ms
Iteration 5: Voxelization: 1.693 ms
Iteration 5: Covariance computation: 42.631 ms
Iteration 5: 42.631 ms
Iteration 6: Voxelization: 1.581 ms
Iteration 6: Covariance computation: 43.114 ms
Iteration 6: 43.114 ms
Iteration 7: Voxelization: 1.702 ms
Iteration 7: Covariance computation: 42.697 ms
Iteration 7: 42.697 ms
Iteration 8: Voxelization: 1.801 ms
Iteration 8: Covariance computation: 42.344 ms
Iteration 8: 42.344 ms
Iteration 9: Voxelization: 1.770 ms
Iteration 9: Covariance computation: 43.115 ms
Iteration 9: 43.115 ms
Iteration 10: Voxelization: 1.745 ms
Iteration 10: Covariance computation: 43.209 ms
Iteration 10: 43.209 ms

=== Benchmark Results ===
Input points:  1413157
Output points: 153903
Voxel size:    0.5

Timing statistics (ms):
  Mean:   43.175
  Median: 43.026
  Min:    42.344
  Max:    46.150
  Stddev: 1.025
```