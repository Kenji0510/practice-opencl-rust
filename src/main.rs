use std::time::Instant;

use anyhow::Result;
use ndarray::Array2;
use ocl::{Device, Platform, core::DeviceInfo};
use practice_ocl::{gpu_voxel::OclVoxelContext, operate_pcd_file::{PointXYZ, PointXYZT, load_pcd_xyz, load_pcd_xyzt}};


const VOXEL_SIZE: f32 = 0.5;
const WARMUP_ITERATIONS: usize = 3;
const BENCHMARK_ITERATIONS: usize = 10;

fn main() -> Result<()> {
    check_device_info()?;

    let pcd_path = "data/input/mid360-pointcloud2-bag-outside-station-to-campus.pcd";
    // let init_pcd = load_pcd_xyzt(pcd_path)
    //     .expect("Failed to load initial PCD file");
    let init_pcd = load_pcd_xyz(pcd_path)
        .expect("Failed to load initial PCD file");

    let init_points = pcd_to_array2(&init_pcd);

    let mut gpu_voxel = OclVoxelContext::new()
        .expect("Failed to create OclVoxelContext");

    println!("\n=== Warming up ({} iterations) ===", WARMUP_ITERATIONS);
    for i in 0..WARMUP_ITERATIONS {
        let (_, valid) = gpu_voxel.voxel_downsample(
            &init_points, 
            init_points.nrows(), 
            VOXEL_SIZE,
        ).expect("Voxel downsample failed");
        println!("Warmup {}: {} output points", i + 1, valid);
    }

    println!("\n=== Benchmarking ({} iterations) ===", BENCHMARK_ITERATIONS);
    let mut times = Vec::with_capacity(BENCHMARK_ITERATIONS);
    let mut valid_count = 0;

    for i in 0..BENCHMARK_ITERATIONS {
        let t_start = Instant::now();
        let (_, valid) = gpu_voxel.voxel_downsample(
            &init_points, 
            init_points.nrows(), 
            VOXEL_SIZE,
        ).expect("Voxel downsample failed");
        let elapsed_ms = t_start.elapsed().as_secs_f64() * 1000.0;
        
        times.push(elapsed_ms);
        valid_count = valid;
        println!("Iteration {}: {:.3} ms", i + 1, elapsed_ms);
    }

    // 統計情報
    let sum: f64 = times.iter().sum();
    let mean = sum / times.len() as f64;
    
    let mut sorted = times.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = sorted[sorted.len() / 2];
    let min = sorted[0];
    let max = sorted[sorted.len() - 1];

    let variance: f64 = times.iter()
        .map(|t| (t - mean).powi(2))
        .sum::<f64>() / times.len() as f64;
    let stddev = variance.sqrt();

    println!("\n=== Benchmark Results ===");
    println!("Input points:  {}", init_points.nrows());
    println!("Output points: {}", valid_count);
    println!("Voxel size:    {}", VOXEL_SIZE);
    println!("\nTiming statistics (ms):");
    println!("  Mean:   {:.3}", mean);
    println!("  Median: {:.3}", median);
    println!("  Min:    {:.3}", min);
    println!("  Max:    {:.3}", max);
    println!("  Stddev: {:.3}", stddev);
    
    Ok(())
}

// fn pcd_to_array2(pcd_points: &[PointXYZT]) -> Array2<f32> {
fn pcd_to_array2(pcd_points: &[PointXYZ]) -> Array2<f32> {
    let n = pcd_points.len();
    let mut arr = Array2::<f32>::zeros((n, 3));
    
    for (i, pt) in pcd_points.iter().enumerate() {
        arr[[i, 0]] = pt.x;
        arr[[i, 1]] = pt.y;
        arr[[i, 2]] = pt.z;
    }
    
    arr
}

fn check_device_info() -> Result<()> {
    for plat in Platform::list() {
        println!("=== Platform: {} ===", plat.name()?);

        let devices = Device::list_all(plat)?;
        for dev in devices {
            let name = dev.name()?;
            let dtype = dev.info(DeviceInfo::Type)?.to_string();
            let version = dev.version()?;

            // 拡張一覧（長い文字列）
            let exts = dev.info(DeviceInfo::Extensions)?.to_string();

            let has_float_atomics = exts.split_whitespace().any(|e| e == "cl_ext_float_atomics");

            println!("Device: {}", name);
            println!("  Type: {}", dtype);
            println!("  Version: {}", version);
            println!("  cl_ext_float_atomics: {}", has_float_atomics);

            // ついでに “似た名前” を含む拡張も拾いたい場合
            let related: Vec<&str> = exts
                .split_whitespace()
                .filter(|e| e.contains("float") && e.contains("atomic"))
                .collect();
            if !related.is_empty() {
                println!("  related: {:?}", related);
            }
            println!();
        }
    }
    Ok(())
}