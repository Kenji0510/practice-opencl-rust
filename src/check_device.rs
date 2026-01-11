use anyhow::Result;
use ocl::{Platform, Device};
use ocl::enums::{DeviceInfo, DeviceInfoResult};


fn yn(b: bool) -> &'static str { if b { "YES" } else { "NO" } }

fn fmt_bytes(bytes: u64) -> String {
    const KIB: f64 = 1024.0;
    const MIB: f64 = 1024.0 * 1024.0;
    const GIB: f64 = 1024.0 * 1024.0 * 1024.0;

    let b = bytes as f64;
    if b >= GIB {
        format!("{:.2} GiB ({} bytes)", b / GIB, bytes)
    } else if b >= MIB {
        format!("{:.2} MiB ({} bytes)", b / MIB, bytes)
    } else if b >= KIB {
        format!("{:.2} KiB ({} bytes)", b / KIB, bytes)
    } else {
        format!("{} bytes", bytes)
    }
}

/// DeviceInfo を安全に取得（対応してない info は Err になるので None）
fn info(dev: &Device, key: DeviceInfo) -> Option<DeviceInfoResult> {
    dev.info(key).ok()
}

fn as_string(v: Option<DeviceInfoResult>) -> Option<String> {
    match v {
        Some(DeviceInfoResult::Name(s)) => Some(s),
        Some(DeviceInfoResult::Vendor(s)) => Some(s),
        Some(DeviceInfoResult::Version(s)) => Some(s.to_string()),
        Some(DeviceInfoResult::DriverVersion(s)) => Some(s),
        Some(DeviceInfoResult::Profile(s)) => Some(s),
        Some(DeviceInfoResult::OpenclCVersion(s)) => Some(s),
        Some(DeviceInfoResult::Extensions(s)) => Some(s),
        Some(other) => Some(format!("{:?}", other)),
        None => None,
    }
}

fn as_u64(v: Option<DeviceInfoResult>) -> Option<u64> {
    match v {
        Some(DeviceInfoResult::MaxComputeUnits(x)) => Some(x as u64),
        Some(DeviceInfoResult::MaxWorkItemDimensions(x)) => Some(x as u64),
        Some(DeviceInfoResult::MaxWorkGroupSize(x)) => Some(x as u64),
        Some(DeviceInfoResult::MaxClockFrequency(x)) => Some(x as u64),
        Some(DeviceInfoResult::GlobalMemSize(x)) => Some(x),
        Some(DeviceInfoResult::LocalMemSize(x)) => Some(x),
        Some(DeviceInfoResult::MaxMemAllocSize(x)) => Some(x),
        Some(DeviceInfoResult::MaxConstantBufferSize(x)) => Some(x),
        // Add other numeric variants as needed
        _ => None,
    }
}

fn as_usize_vec(v: Option<DeviceInfoResult>) -> Option<Vec<usize>> {
    match v {
        Some(DeviceInfoResult::MaxWorkItemSizes(vec)) => Some(vec),
        _ => None,
    }
}

fn print_kv(name: &str, val: Option<String>) {
    if let Some(v) = val {
        println!("  {:32}: {}", name, v);
    } else {
        println!("  {:32}: N/A", name);
    }
}

fn print_bytes(name: &str, val: Option<u64>) {
    if let Some(v) = val {
        println!("  {:32}: {}", name, fmt_bytes(v));
    } else {
        println!("  {:32}: N/A", name);
    }
}

fn print_u64(name: &str, val: Option<u64>) {
    if let Some(v) = val {
        println!("  {:32}: {}", name, v);
    } else {
        println!("  {:32}: N/A", name);
    }
}

fn ext_has(exts: &str, ext: &str) -> bool {
    exts.split_whitespace().any(|e| e == ext)
}


pub fn check_device_info() -> Result<()> {
    let platforms = Platform::list();

    if platforms.is_empty() {
        println!("No OpenCL platform found.");
        return Ok(());
    }

    for (pi, p) in platforms.iter().enumerate() {
        println!();
        println!("============================================================");
        println!("[Platform {}] {:?}", pi, p);

        // Platform info（可能なら）
        // ocl は PlatformInfo もあるけど、ここでは簡潔に。
        let devices = Device::list_all(*p)?;
        if devices.is_empty() {
            println!("  (No devices)");
            continue;
        }

        for (di, dev) in devices.iter().enumerate() {
            println!();
            println!("------------------------------------------------------------");
            let name = as_string(info(dev, DeviceInfo::Name)).unwrap_or_else(|| "(unknown)".into());
            println!("[Device {}] {}", di, name);

            print_kv("vendor", as_string(info(dev, DeviceInfo::Vendor)));
            print_kv("version", as_string(info(dev, DeviceInfo::Version)));
            print_kv("driver_version", as_string(info(dev, DeviceInfo::DriverVersion)));
            print_kv("profile", as_string(info(dev, DeviceInfo::Profile)));
            print_kv("opencl_c_version", as_string(info(dev, DeviceInfo::OpenclCVersion)));

            // 基本性能・実行系
            println!();
            println!("-- Compute --");
            print_u64("max_compute_units", as_u64(info(dev, DeviceInfo::MaxComputeUnits)));
            print_u64("max_clock_frequency_mhz", as_u64(info(dev, DeviceInfo::MaxClockFrequency)));
            print_u64("address_bits", as_u64(info(dev, DeviceInfo::AddressBits)));

            // Work-group / Work-item
            println!();
            println!("-- Work-group / Work-item --");
            print_u64("max_work_group_size", as_u64(info(dev, DeviceInfo::MaxWorkGroupSize)));
            if let Some(dims) = as_u64(info(dev, DeviceInfo::MaxWorkItemDimensions)) {
                println!("  {:32}: {}", "max_work_item_dimensions", dims);
            } else {
                println!("  {:32}: N/A", "max_work_item_dimensions");
            }
            if let Some(sizes) = as_usize_vec(info(dev, DeviceInfo::MaxWorkItemSizes)) {
                println!("  {:32}: {:?}", "max_work_item_sizes", sizes);
            } else {
                println!("  {:32}: N/A", "max_work_item_sizes");
            }

            // メモリ上限（あなたの “一度に扱えるメモリ” に相当）
            println!();
            println!("-- Memory / Limits --");
            print_bytes("global_mem_size", as_u64(info(dev, DeviceInfo::GlobalMemSize)));
            print_bytes("max_mem_alloc_size", as_u64(info(dev, DeviceInfo::MaxMemAllocSize)));
            print_bytes("__local (local_mem_size)", as_u64(info(dev, DeviceInfo::LocalMemSize)));
            print_bytes("max_constant_buffer_size", as_u64(info(dev, DeviceInfo::MaxConstantBufferSize)));

            // ついで：アライメント/引数サイズ（地味に重要）
            print_u64("mem_base_addr_align_bits", as_u64(info(dev, DeviceInfo::MemBaseAddrAlign)));
            print_u64("min_data_type_align_size", as_u64(info(dev, DeviceInfo::MinDataTypeAlignSize)));
            print_u64("max_parameter_size", as_u64(info(dev, DeviceInfo::MaxParameterSize)));

            // // 画像（必要なら）
            // println!();
            // println!("-- Images (optional) --");
            // print_kv("image_support", as_string(info(dev, DeviceInfo::ImageSupport)));
            // print_u64("image2d_max_width", as_u64(info(dev, DeviceInfo::Image2dMaxWidth)));
            // print_u64("image2d_max_height", as_u64(info(dev, DeviceInfo::Image2dMaxHeight)));

            // 拡張から “atomic 対応範囲” を推定（OpenCL流）
            println!();
            println!("-- Atomics / Integer capabilities (from extensions) --");
            let exts = as_string(info(dev, DeviceInfo::Extensions)).unwrap_or_default();

            // int32 atomics（global/local）
            let g_i32_base = ext_has(&exts, "cl_khr_global_int32_base_atomics");
            let g_i32_ext  = ext_has(&exts, "cl_khr_global_int32_extended_atomics");
            let l_i32_base = ext_has(&exts, "cl_khr_local_int32_base_atomics");
            let l_i32_ext  = ext_has(&exts, "cl_khr_local_int32_extended_atomics");

            // int64 atomics（base/extended）
            let i64_base = ext_has(&exts, "cl_khr_int64_base_atomics");
            let i64_ext  = ext_has(&exts, "cl_khr_int64_extended_atomics");

            // float atomics（環境依存で有無が分かれる）
            let f_atomic = ext_has(&exts, "cl_ext_float_atomics")
                || ext_has(&exts, "cl_khr_fp16") // これは float16 型そのもの用
                || ext_has(&exts, "cl_khr_fp64"); // float64 型そのもの用（atomicではないが参考）

            println!("  global int32 base atomics          : {}", yn(g_i32_base));
            println!("  global int32 extended atomics      : {}", yn(g_i32_ext));
            println!("  local  int32 base atomics          : {}", yn(l_i32_base));
            println!("  local  int32 extended atomics      : {}", yn(l_i32_ext));

            println!("  int64 base atomics                 : {}", yn(i64_base));
            println!("  int64 extended atomics             : {}", yn(i64_ext));

            // atomicCAS 相当の目安：extended atomics があるかどうか
            // （厳密には演算セットや対象アドレス空間で差があるので “目安” 表示）
            println!();
            println!("-- Notes (atomicCAS / compare-exchange) --");
            println!("  int32 atomicCAS-ish (extended)     : {}", yn(g_i32_ext || l_i32_ext));
            println!("  int64 atomicCAS-ish (extended)     : {}", yn(i64_ext));

            println!();
            println!("-- Floating / 16-64bit types (hints) --");
            println!("  float atomics extension present    : {}", yn(f_atomic));
            println!("  fp16 type present (cl_khr_fp16)    : {}", yn(ext_has(&exts, "cl_khr_fp16")));
            println!("  fp64 type present (cl_khr_fp64)    : {}", yn(ext_has(&exts, "cl_khr_fp64")));

            // 便利：生の extensions も最後に出す（長いので必要ならコメントアウト）
            println!();
            println!("-- Raw extensions --");
            println!("  {}", exts);
        }
    }

    Ok(())
}