# RTX4080 (Ubuntu22)
```bash
============================================================
[Platform 0] Platform(PlatformId(0x5d1ef8bd0ba0))

------------------------------------------------------------
[Device 0] NVIDIA GeForce RTX 4080
  vendor                          : NVIDIA Corporation
  version                         : 3.0
  driver_version                  : 580.95.05
  profile                         : FULL_PROFILE
  opencl_c_version                : OpenCL C 1.2

-- Compute --
  max_compute_units               : 76
  max_clock_frequency_mhz         : 2565
  address_bits                    : N/A

-- Work-group / Work-item --
  max_work_group_size             : 1024
  max_work_item_dimensions        : 3
  max_work_item_sizes             : [1024, 1024, 64]

-- Memory / Limits --
  global_mem_size                 : 15.57 GiB (16718168064 bytes)
  max_mem_alloc_size              : 3.89 GiB (4179542016 bytes)
  __local (local_mem_size)        : 48.00 KiB (49152 bytes)
  max_constant_buffer_size        : 64.00 KiB (65536 bytes)
  mem_base_addr_align_bits        : N/A
  min_data_type_align_size        : N/A
  max_parameter_size              : N/A

-- Images (optional) --
  image_support                   : true
  image2d_max_width               : N/A
  image2d_max_height              : N/A

-- Atomics / Integer capabilities (from extensions) --
  global int32 base atomics          : YES
  global int32 extended atomics      : YES
  local  int32 base atomics          : YES
  local  int32 extended atomics      : YES
  int64 base atomics                 : YES
  int64 extended atomics             : YES

-- Notes (atomicCAS / compare-exchange) --
  int32 atomicCAS-ish (extended)     : YES
  int64 atomicCAS-ish (extended)     : YES

-- Floating / 16-64bit types (hints) --
  float atomics extension present    : YES
  fp16 type present (cl_khr_fp16)    : NO
  fp64 type present (cl_khr_fp64)    : YES

-- Raw extensions --
  cl_khr_global_int32_base_atomics cl_khr_global_int32_extended_atomics cl_khr_local_int32_base_atomics cl_khr_local_int32_extended_atomics cl_khr_fp64 cl_khr_3d_image_writes cl_khr_byte_addressable_store cl_khr_icd cl_khr_gl_sharing cl_nv_compiler_options cl_nv_device_attribute_query cl_nv_pragma_unroll cl_nv_copy_opts cl_nv_create_buffer cl_khr_int64_base_atomics cl_khr_int64_extended_atomics cl_khr_device_uuid cl_khr_pci_bus_info cl_khr_external_semaphore cl_khr_external_memory cl_khr_external_semaphore_opaque_fd cl_khr_external_memory_opaque_fd cl_khr_semaphore
```