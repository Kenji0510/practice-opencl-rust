extern crate ocl;
use std::time::Instant;

use ocl::{Buffer, Kernel, MemFlags, ProQue};
use anyhow::{Context, Result};


const WORK_SIZE: usize = 1 << 24;
const ITERS: usize = 1000;
const PRINT: usize = 8;

static KERNEL_SRC: &str = r#"
    __kernel void add(
        __global const float* a,
        __global const float* b,
        __global float* c,
        uint iters
    ) {
        uint idx = get_global_id(0);
        
        float ai = a[idx];
        float bi = b[idx];
        float x = c[idx];

        for(uint k = 0; k < iters; ++k) {
            x = fma(bi, x, ai);
        }
        c[idx] = x;
    }
"#;

fn main() -> Result<()> {
    let pq = ProQue::builder()
        .src(KERNEL_SRC)
        .dims(WORK_SIZE)
        .build()
        .context("Failed to create Context...")?;

    let a = vec![1e-3f32; WORK_SIZE];
    let b = vec![0.999f32; WORK_SIZE];
    let mut c = vec![1.0f32; WORK_SIZE];

    let buf_a = Buffer::<f32>::builder()
        .queue(pq.queue().clone())
        .flags(MemFlags::new().read_only())
        .len(WORK_SIZE)
        .copy_host_slice(&a)
        .build()
        .context("Failed to create buffer A...")?;

    let buf_b = Buffer::<f32>::builder()
        .queue(pq.queue().clone())
        .flags(MemFlags::new().read_only())
        .len(WORK_SIZE)
        .copy_host_slice(&b)
        .build()
        .context("Failed to create buffer B...")?;

    let buf_c = Buffer::<f32>::builder()
        .queue(pq.queue().clone())
        .flags(MemFlags::new().read_write())
        .len(WORK_SIZE)
        .copy_host_slice(&c)
        .build()
        .context("Failed to create buffer C...")?;

    let kernel = Kernel::builder()
        .program(&pq.program())
        .name("add")
        .queue(pq.queue().clone())
        .global_work_size(WORK_SIZE)
        .arg(&buf_a)
        .arg(&buf_b)
        .arg(&buf_c)
        .arg(7 as u32)
        .build()
        .context("Failed to create the kernel")?;

    unsafe {
        kernel.enq()
            .context("Failed to run the kernel")?;
    }
    pq.queue().finish().context("Failed to finish the queue")?;

    let t0 = Instant::now();
    for _ in 0..ITERS {
        unsafe { kernel.enq()?; }
    }
    pq.queue().finish()?; 
    let elapsed = t0.elapsed();

    buf_c.read(&mut c).enq()?;
    pq.queue().finish()?;
    
    let mut expected = 1.0f32;
    for _ in 0..ITERS {
        expected = 1e-3 + 0.999 * expected;
    }
    for i in 0..PRINT {
        println!("c[{i}] = {}, expected ~ {}", c[i], expected);
    }

    let total_ops = (WORK_SIZE as f64) * (ITERS as f64) * 2.0;
    let gflops = total_ops / elapsed.as_secs_f64() / 1e9;
    println!("WORK_SIZE={}, ITERS={}, elapsed={:?}, ~{:.2} GFLOP/s", WORK_SIZE, ITERS, elapsed, gflops);

    Ok(())
}