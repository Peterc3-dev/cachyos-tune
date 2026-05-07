use std::time::{Duration, Instant};

use crate::style;

/// Quick CPU benchmark: iterate SHA-256-like computation.
pub fn run_bench() {
    style::header("Quick Benchmark");

    println!("  Running CPU throughput test...");
    let cpu_score = bench_cpu();
    println!(
        "  {} CPU score: {}{:.0}{} ops/sec",
        style::green(">>"),
        "\x1b[1m\x1b[38;2;57;255;20m",
        cpu_score,
        "\x1b[0m"
    );

    println!("\n  Running memory bandwidth test...");
    let mem_bw = bench_memory();
    println!(
        "  {} Memory bandwidth: {}{:.1}{} GB/s",
        style::green(">>"),
        "\x1b[1m\x1b[38;2;57;255;20m",
        mem_bw,
        "\x1b[0m"
    );

    println!("\n  Running sequential read test...");
    let read_speed = bench_seq_read();
    println!(
        "  {} Sequential read: {}{:.1}{} GB/s",
        style::green(">>"),
        "\x1b[1m\x1b[38;2;57;255;20m",
        read_speed,
        "\x1b[0m"
    );

    println!(
        "\n  {}Tip: run with different profiles to compare.{}",
        "\x1b[38;2;30;140;10m", "\x1b[0m"
    );
    println!();
}

/// CPU benchmark: tight arithmetic loop.
fn bench_cpu() -> f64 {
    let duration = Duration::from_secs(2);
    let start = Instant::now();
    let mut ops: u64 = 0;
    let mut x: u64 = 0x517cc1b727220a95;

    while start.elapsed() < duration {
        for _ in 0..10000 {
            // Fast hash-like mixing
            x = x.wrapping_mul(0x2545F4914F6CDD1D);
            x ^= x >> 28;
            x = x.wrapping_add(0x9E3779B97F4A7C15);
            // Prevent auto-vectorization from skewing results
            x = std::hint::black_box(x);
        }
        ops += 10000;
    }

    let elapsed = start.elapsed().as_secs_f64();
    ops as f64 / elapsed
}

/// Memory bandwidth benchmark: sequential write to a large buffer.
fn bench_memory() -> f64 {
    let size = 64 * 1024 * 1024; // 64 MB
    let mut buf = vec![0u8; size];
    let iterations = 10;

    let start = Instant::now();
    for iter in 0..iterations {
        let val = iter as u8;
        // Write in 8-byte chunks for better throughput measurement
        for chunk in buf.chunks_exact_mut(8) {
            chunk[0] = val;
            chunk[1] = val.wrapping_add(1);
            chunk[2] = val.wrapping_add(2);
            chunk[3] = val.wrapping_add(3);
            chunk[4] = val.wrapping_add(4);
            chunk[5] = val.wrapping_add(5);
            chunk[6] = val.wrapping_add(6);
            chunk[7] = val.wrapping_add(7);
        }
    }
    std::hint::black_box(&buf);

    let elapsed = start.elapsed().as_secs_f64();
    let total_bytes = size as f64 * iterations as f64;
    total_bytes / elapsed / 1e9
}

/// Sequential read benchmark: read from a large buffer repeatedly.
fn bench_seq_read() -> f64 {
    let size = 64 * 1024 * 1024; // 64 MB
    let buf: Vec<u8> = (0..size).map(|i| (i & 0xFF) as u8).collect();
    let iterations = 20;

    let start = Instant::now();
    let mut checksum: u64 = 0;
    for _ in 0..iterations {
        for chunk in buf.chunks_exact(8) {
            let val = u64::from_le_bytes([
                chunk[0], chunk[1], chunk[2], chunk[3],
                chunk[4], chunk[5], chunk[6], chunk[7],
            ]);
            checksum = checksum.wrapping_add(val);
        }
    }
    std::hint::black_box(checksum);

    let elapsed = start.elapsed().as_secs_f64();
    let total_bytes = size as f64 * iterations as f64;
    total_bytes / elapsed / 1e9
}
