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

=== Warning up (3 iterations) ===
Warmup 1: 1057 output points
Warmup 2: 1057 output points
Warmup 3: 1057 output points

=== Benchmarking (10 iterations) ===
Iteration 1: Voxelization: 0.051 ms
Iteration 1: Covariance computation: 0.262 ms
Iteration 1: Nearest neighbor search: 0.318 ms
Iteration 1: Transform points: 0.333 ms
Iteration 1: 0.333 ms
Iteration 2: Voxelization: 0.052 ms
Iteration 2: Covariance computation: 0.263 ms
Iteration 2: Nearest neighbor search: 0.305 ms
Iteration 2: Transform points: 0.316 ms
Iteration 2: 0.316 ms
Iteration 3: Voxelization: 0.071 ms
Iteration 3: Covariance computation: 0.285 ms
Iteration 3: Nearest neighbor search: 0.328 ms
Iteration 3: Transform points: 0.339 ms
Iteration 3: 0.339 ms
Iteration 4: Voxelization: 0.055 ms
Iteration 4: Covariance computation: 0.263 ms
Iteration 4: Nearest neighbor search: 0.304 ms
Iteration 4: Transform points: 0.314 ms
Iteration 4: 0.314 ms
Iteration 5: Voxelization: 0.052 ms
Iteration 5: Covariance computation: 0.260 ms
Iteration 5: Nearest neighbor search: 0.301 ms
Iteration 5: Transform points: 0.310 ms
Iteration 5: 0.310 ms
Iteration 6: Voxelization: 0.063 ms
Iteration 6: Covariance computation: 0.270 ms
Iteration 6: Nearest neighbor search: 0.313 ms
Iteration 6: Transform points: 0.323 ms
Iteration 6: 0.323 ms
Iteration 7: Voxelization: 0.053 ms
Iteration 7: Covariance computation: 0.261 ms
Iteration 7: Nearest neighbor search: 0.303 ms
Iteration 7: Transform points: 0.312 ms
Iteration 7: 0.312 ms
Iteration 8: Voxelization: 0.051 ms
Iteration 8: Covariance computation: 0.261 ms
Iteration 8: Nearest neighbor search: 0.302 ms
Iteration 8: Transform points: 0.311 ms
Iteration 8: 0.311 ms
Iteration 9: Voxelization: 0.052 ms
Iteration 9: Covariance computation: 0.262 ms
Iteration 9: Nearest neighbor search: 0.303 ms
Iteration 9: Transform points: 0.312 ms
Iteration 9: 0.312 ms
Iteration 10: Voxelization: 0.051 ms
Iteration 10: Covariance computation: 0.260 ms
Iteration 10: Nearest neighbor search: 0.302 ms
Iteration 10: Transform points: 0.312 ms
Iteration 10: 0.312 ms

=== Benchmark Results ===
Input points:  19968
Output points: 1057
Voxel size:    0.5

Timing statistics (ms):
  Mean:   0.318
  Median: 0.314
  Min:    0.310
  Max:    0.339
  Stddev: 0.009
```