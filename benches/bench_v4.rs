use std::hint::black_box;
use std::time::{Duration, Instant};
use nabu::{XffValue, Array, Object, Data, Graph};
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

    run_bench(
        "primitives",
        "Ser",
        50000,
        || {
            serialize_xff_to_buffer(vec![val_primitives.clone()], 4).unwrap()
        },
        None
    ).print();

    run_bench(
        "primitives",
        "Deser",
        50000,
        || {
            deserialize_xff_from_buffer(&buf_primitives).unwrap()
        },
        None
    ).print();

    // Case 2: Large Array (1000 integers)
    let large_arr_values: Vec<XffValue> = (0..1000).map(|i| XffValue::from(i)).collect();
    let val_large_arr = XffValue::Array(Array::from(large_arr_values));
    let buf_large_arr = serialize_xff_to_buffer(vec![val_large_arr.clone()], 4).expect("Failed serialization");

    run_bench(
        "large_array",
        "Ser",
        1000,
        || {
            serialize_xff_to_buffer(vec![val_large_arr.clone()], 4).unwrap()
        },
        Some(buf_large_arr.len())
    ).print();

    run_bench(
        "large_array",
        "Deser",
        1000,
        || {
            deserialize_xff_from_buffer(&buf_large_arr).unwrap()
        },
        Some(buf_large_arr.len())
    ).print();

    // Case 3: Large Object (100 key-value pairs)
    let mut large_obj = Object::new();
    for i in 0..100 {
        large_obj.insert(format!("key_{}", i), XffValue::from(i));
    }
    let val_large_obj = XffValue::Object(large_obj);
    let buf_large_obj = serialize_xff_to_buffer(vec![val_large_obj.clone()], 4).expect("Failed serialization");

    run_bench(
        "large_object",
        "Ser",
        1000,
        || {
            serialize_xff_to_buffer(vec![val_large_obj.clone()], 4).unwrap()
        },
        Some(buf_large_obj.len())
    ).print();

    run_bench(
        "large_object",
        "Deser",
        1000,
        || {
            deserialize_xff_from_buffer(&buf_large_obj).unwrap()
        },
        Some(buf_large_obj.len())
    ).print();

    // Case 4: Large Data (1 MB binary payload)
    let raw_data = vec![127u8; 1024 * 1024];
    let val_large_data = XffValue::Data(Data::from(raw_data));
    let buf_large_data = serialize_xff_to_buffer(vec![val_large_data.clone()], 4).expect("Failed serialization");

    run_bench(
        "large_data (1 MB)",
        "Ser",
        100,
        || {
            serialize_xff_to_buffer(vec![val_large_data.clone()], 4).unwrap()
        },
        Some(buf_large_data.len())
    ).print();

    run_bench(
        "large_data (1 MB)",
        "Deser",
        100,
        || {
            deserialize_xff_from_buffer(&buf_large_data).unwrap()
        },
        Some(buf_large_data.len())
    ).print();

    // Case 5: Graph (50 nodes, 100 connections)
    let mut graph = Graph::new();
    let nodes: Vec<_> = (0..50)
        .map(|i| graph.add_node(XffValue::from(format!("node_{}", i)), XffValue::Null))
        .collect();
    for i in 0..50 {
        let from = nodes[i];
        let to = nodes[(i + 1) % 50];
        let to_other = nodes[(i + 25) % 50];
        let _ = graph.add_connection(from, to, XffValue::from(format!("edge_{}_to_{}", i, (i + 1) % 50)));
        let _ = graph.add_connection(from, to_other, XffValue::from(format!("edge_{}_to_{}", i, (i + 25) % 50)));
    }
    let val_graph = XffValue::Graph(graph);
    let buf_graph = serialize_xff_to_buffer(vec![val_graph.clone()], 4).expect("Failed serialization");

    run_bench(
        "graph",
        "Ser",
        500,
        || {
            serialize_xff_to_buffer(vec![val_graph.clone()], 4).unwrap()
        },
        Some(buf_graph.len())
    ).print();

    run_bench(
        "graph",
        "Deser",
        500,
        || {
            deserialize_xff_from_buffer(&buf_graph).unwrap()
        },
        Some(buf_graph.len())
    ).print();
}
