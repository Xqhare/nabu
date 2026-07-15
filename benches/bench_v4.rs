use std::hint::black_box;
use std::time::{Duration, Instant};
use nabu::{XffValue, Number, Array, Object, Data, Graph};
use nabu::serde::{serialize_xff_to_buffer, deserialize_xff_from_buffer};

struct BenchResult {
    name: &'static str,
    mode: &'static str,
    iterations: usize,
    total_time: Duration,
    bytes: Option<usize>,
}

impl BenchResult {
    fn print(&self) {
        let time_per_op = self.total_time.as_secs_f64() / (self.iterations as f64);
        let time_str = if time_per_op < 1e-6 {
            format!("{:.1} ns", time_per_op * 1e9)
        } else if time_per_op < 1e-3 {
            format!("{:.1} µs", time_per_op * 1e6)
        } else {
            format!("{:.1} ms", time_per_op * 1e3)
        } ;

        let throughput = if let Some(bytes) = self.bytes {
            let total_bytes = bytes * self.iterations;
            let mb_per_sec = (total_bytes as f64 / (1024.0 * 1024.0)) / self.total_time.as_secs_f64();
            format!("{:.2} MB/s", mb_per_sec)
        } else {
            "N/A".to_string()
        };

        println!(
            "{:<30} {:<8} {:<10} {:<15} {:<15} {:<15}",
            self.name,
            self.mode,
            self.iterations,
            format!("{:.2} ms", self.total_time.as_secs_f64() * 1000.0),
            time_str,
            throughput
        );
    }
}

fn run_bench<F, R>(name: &'static str, mode: &'static str, iterations: usize, mut op: F, bytes: Option<usize>) -> BenchResult
where
    F: FnMut() -> R,
{
    // Warmup
    for _ in 0..100 {
        let _ = black_box(op());
    }

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = black_box(op());
    }
    let total_time = start.elapsed();

    BenchResult {
        name,
        mode,
        iterations,
        total_time,
        bytes,
    }
}

fn main() {
    println!(
        "\n{:<30} {:<8} {:<10} {:<15} {:<15} {:<15}",
        "Benchmark", "Mode", "Iter", "Total Time", "Time/Op", "Throughput"
    );
    println!("{}", "-".repeat(98));

    // Case 1: Primitives
    let primitives = vec![
        XffValue::Null,
        XffValue::from(true),
        XffValue::from(false),
        XffValue::NaN,
        XffValue::PosNaN,
        XffValue::NegNaN,
        XffValue::Infinity,
        XffValue::NegInfinity,
        XffValue::from(42),
        XffValue::from(-1234),
    ];
    let val_primitives = XffValue::Array(Array::from(primitives));
    let buf_primitives = serialize_xff_to_buffer(vec![val_primitives.clone()], 4).expect("Failed serialization");

    let bench_ser_primitives = run_bench(
        "primitives",
        "Ser",
        50000,
        || {
            serialize_xff_to_buffer(vec![val_primitives.clone()], 4).unwrap()
        },
        None
    );
    bench_ser_primitives.print();

    let bench_deser_primitives = run_bench(
        "primitives",
        "Deser",
        50000,
        || {
            deserialize_xff_from_buffer(&buf_primitives).unwrap()
        },
        None
    );
    bench_deser_primitives.print();
}
