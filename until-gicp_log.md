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
Warmup 1: 2439 output points
Warmup 2: 2439 output points
Warmup 3: 2439 output points

=== Benchmarking (10 iterations) ===
Iteration 1: Voxelization: 0.145 ms
Iteration 1: Covariance computation: 0.235 ms
Iteration 1: Transform points: 0.022 ms
Iteration 1: Nearest neighbor search: 0.045 ms
Iteration 1: GICP computation: 0.043 ms
Iteration 1: 0.512 ms
Iteration 0: RMSE = 0.58259165
Iteration 2: Voxelization: 0.163 ms
Iteration 2: Covariance computation: 0.237 ms
Iteration 2: Transform points: 0.008 ms
Iteration 2: Nearest neighbor search: 0.041 ms
Iteration 2: GICP computation: 0.038 ms
Iteration 2: 0.507 ms
Iteration 1: RMSE = 0.5825916
Iteration 3: Voxelization: 0.163 ms
Iteration 3: Covariance computation: 0.237 ms
Iteration 3: Transform points: 0.008 ms
Iteration 3: Nearest neighbor search: 0.040 ms
Iteration 3: GICP computation: 0.038 ms
Iteration 3: 0.507 ms
Iteration 2: RMSE = 0.5825917
Iteration 4: Voxelization: 0.170 ms
Iteration 4: Covariance computation: 0.237 ms
Iteration 4: Transform points: 0.008 ms
Iteration 4: Nearest neighbor search: 0.044 ms
Iteration 4: GICP computation: 0.037 ms
Iteration 4: 0.519 ms
Iteration 3: RMSE = 0.58259153
Iteration 5: Voxelization: 0.159 ms
Iteration 5: Covariance computation: 0.237 ms
Iteration 5: Transform points: 0.008 ms
Iteration 5: Nearest neighbor search: 0.041 ms
Iteration 5: GICP computation: 0.038 ms
Iteration 5: 0.504 ms
Iteration 4: RMSE = 0.58259153
Iteration 6: Voxelization: 0.160 ms
Iteration 6: Covariance computation: 0.237 ms
Iteration 6: Transform points: 0.008 ms
Iteration 6: Nearest neighbor search: 0.044 ms
Iteration 6: GICP computation: 0.038 ms
Iteration 6: 0.513 ms
Iteration 5: RMSE = 0.58259153
Iteration 7: Voxelization: 0.160 ms
Iteration 7: Covariance computation: 0.237 ms
Iteration 7: Transform points: 0.007 ms
Iteration 7: Nearest neighbor search: 0.039 ms
Iteration 7: GICP computation: 0.038 ms
Iteration 7: 0.503 ms
Iteration 6: RMSE = 0.58259153
Iteration 8: Voxelization: 0.161 ms
Iteration 8: Covariance computation: 0.238 ms
Iteration 8: Transform points: 0.008 ms
Iteration 8: Nearest neighbor search: 0.042 ms
Iteration 8: GICP computation: 0.038 ms
Iteration 8: 0.517 ms
Iteration 7: RMSE = 0.5825916
Iteration 9: Voxelization: 0.164 ms
Iteration 9: Covariance computation: 0.238 ms
Iteration 9: Transform points: 0.008 ms
Iteration 9: Nearest neighbor search: 0.040 ms
Iteration 9: GICP computation: 0.038 ms
Iteration 9: 0.517 ms
Iteration 8: RMSE = 0.58259153
Iteration 10: Voxelization: 0.163 ms
Iteration 10: Covariance computation: 0.238 ms
Iteration 10: Transform points: 0.008 ms
Iteration 10: Nearest neighbor search: 0.041 ms
Iteration 10: GICP computation: 0.038 ms
Iteration 10: 0.509 ms
Iteration 9: RMSE = 0.58259165

=== Benchmark Results ===
Input points:  64764
Output points: 2439
Voxel size:    0.5

Timing statistics (ms):
  Mean:   0.511
  Median: 0.512
  Min:    0.503
  Max:    0.519
  Stddev: 0.005
```
