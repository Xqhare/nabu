use nabu::serde::{deserialize_xff_from_buffer, serialize_xff_to_buffer};
use nabu::{Array, Data, Graph, Object, XffValue};
use std::hint::black_box;
use std::time::{Duration, Instant};

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
        };

        let throughput = if let Some(bytes) = self.bytes {
            let total_bytes = bytes * self.iterations;
            let mb_per_sec =
                (total_bytes as f64 / (1024.0 * 1024.0)) / self.total_time.as_secs_f64();
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

fn run_bench<F, R>(
    name: &'static str,
    mode: &'static str,
    iterations: usize,
    mut op: F,
    bytes: Option<usize>,
) -> BenchResult
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

fn create_complex_mock() -> XffValue {
    // 1. Level 4: Profile and Prefs
    let mut preferences = Object::new();
    preferences.insert("theme".to_string(), XffValue::from("dark"));
    preferences.insert("notifications".to_string(), XffValue::from(true));
    preferences.insert("retry_limit".to_string(), XffValue::from(5));

    let login_history = XffValue::Array(Array::from(
        (0..10).map(|i| XffValue::from(1721081700 + i * 3600)).collect::<Vec<_>>()
    ));

    // Large 1 KB string for bio
    let large_bio = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. ".repeat(18); // ~1026 bytes
    let mut profile = Object::new();
    profile.insert("bio".to_string(), XffValue::from(large_bio));
    profile.insert("preferences".to_string(), XffValue::Object(preferences));
    profile.insert("login_history".to_string(), login_history);

    // 2. Level 3: Users Array
    let mut users_list = Vec::with_capacity(50);
    for i in 0..50 {
        let mut user = Object::new();
        user.insert("id".to_string(), XffValue::Uuid(nabu::Uuid::new([i as u8; 16])));
        user.insert("username".to_string(), XffValue::Ascii(athena::XffString::from(format!("user_{}", i))));
        user.insert("permissions".to_string(), XffValue::Array(Array::from(vec![
            XffValue::from("read"),
            XffValue::from("write"),
        ])));
        user.insert("profile".to_string(), XffValue::Object(profile.clone()));
        users_list.push(XffValue::Object(user));
    }
    let users_array = XffValue::Array(Array::from(users_list));

    // 3. Level 2: Metrics Table
    let columns = vec![
        "metric_name".to_string(),
        "value".to_string(),
        "status".to_string(),
    ];
    let mut rows = Vec::with_capacity(10);
    for i in 0..10 {
        rows.push(vec![
            XffValue::from(format!("metric_{}", i)),
            XffValue::from(42.0 + (i as f64) * 1.5),
            XffValue::from(i % 2 == 0),
        ]);
    }
    let metrics_table = XffValue::Table(nabu::Table { columns, rows });

    // 4. Level 2: Network Graph
    let mut graph = Graph::new();
    let nodes: Vec<_> = (0..5)
        .map(|i| graph.add_node(XffValue::from(format!("server_node_{}", i)), XffValue::Null))
        .collect();
    for i in 0..5 {
        let from = nodes[i];
        let to = nodes[(i + 1) % 5];
        let _ = graph.add_connection(from, to, XffValue::from(format!("link_{}_to_{}", i, (i + 1) % 5)));
    }
    let network_graph = XffValue::Graph(graph);

    // 5. Level 2: Config Server details
    let mut server = Object::new();
    server.insert("host".to_string(), XffValue::from("127.0.0.1"));
    server.insert("port".to_string(), XffValue::from(8080));
    server.insert("timeout".to_string(), XffValue::from(30.5));

    let mut config = Object::new();
    config.insert("server".to_string(), XffValue::Object(server));
    config.insert("users".to_string(), users_array);
    config.insert("metrics".to_string(), metrics_table);
    config.insert("network".to_string(), network_graph);

    // 6. Level 1: Root Object
    let mut metadata = Object::new();
    metadata.insert("creator".to_string(), XffValue::from("bench_runner"));
    metadata.insert("version".to_string(), XffValue::from("4.1.0"));

    let mut system_status = Object::new();
    system_status.insert("active".to_string(), XffValue::from(true));
    system_status.insert("code".to_string(), XffValue::from(200));
    system_status.insert("msg".to_string(), XffValue::from("healthy"));

    let mut root = Object::new();
    root.insert("metadata".to_string(), XffValue::Metadata(nabu::Metadata::from(metadata)));
    root.insert("config".to_string(), XffValue::Object(config));
    root.insert("system_status".to_string(), XffValue::Object(system_status));

    XffValue::Object(root)
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
    let buf_primitives =
        serialize_xff_to_buffer(vec![val_primitives.clone()], 4).expect("Failed serialization");

    run_bench(
        "primitives",
        "Ser",
        50000,
        || serialize_xff_to_buffer(vec![val_primitives.clone()], 4).unwrap(),
        None,
    )
    .print();

    run_bench(
        "primitives",
        "Deser",
        50000,
        || deserialize_xff_from_buffer(&buf_primitives).unwrap(),
        None,
    )
    .print();

    // Case 2: Large Array (1000 integers)
    let large_arr_values: Vec<XffValue> = (0..1000).map(|i| XffValue::from(i)).collect();
    let val_large_arr = XffValue::Array(Array::from(large_arr_values));
    let buf_large_arr =
        serialize_xff_to_buffer(vec![val_large_arr.clone()], 4).expect("Failed serialization");

    run_bench(
        "large_array",
        "Ser",
        1000,
        || serialize_xff_to_buffer(vec![val_large_arr.clone()], 4).unwrap(),
        Some(buf_large_arr.len()),
    )
    .print();

    run_bench(
        "large_array",
        "Deser",
        1000,
        || deserialize_xff_from_buffer(&buf_large_arr).unwrap(),
        Some(buf_large_arr.len()),
    )
    .print();

    // Case 3: Large Object (100 key-value pairs)
    let mut large_obj = Object::new();
    for i in 0..100 {
        large_obj.insert(format!("key_{}", i), XffValue::from(i));
    }
    let val_large_obj = XffValue::Object(large_obj);
    let buf_large_obj =
        serialize_xff_to_buffer(vec![val_large_obj.clone()], 4).expect("Failed serialization");

    run_bench(
        "large_object",
        "Ser",
        1000,
        || serialize_xff_to_buffer(vec![val_large_obj.clone()], 4).unwrap(),
        Some(buf_large_obj.len()),
    )
    .print();

    run_bench(
        "large_object",
        "Deser",
        1000,
        || deserialize_xff_from_buffer(&buf_large_obj).unwrap(),
        Some(buf_large_obj.len()),
    )
    .print();

    // Case 4: Large Data (1 MB binary payload)
    let raw_data = vec![127u8; 1024 * 1024];
    let val_large_data = XffValue::Data(Data::from(raw_data));
    let buf_large_data =
        serialize_xff_to_buffer(vec![val_large_data.clone()], 4).expect("Failed serialization");

    run_bench(
        "large_data (1 MB)",
        "Ser",
        100,
        || serialize_xff_to_buffer(vec![val_large_data.clone()], 4).unwrap(),
        Some(buf_large_data.len()),
    )
    .print();

    run_bench(
        "large_data (1 MB)",
        "Deser",
        100,
        || deserialize_xff_from_buffer(&buf_large_data).unwrap(),
        Some(buf_large_data.len()),
    )
    .print();

    // Case 5: Graph (50 nodes, 100 connections)
    let mut graph = Graph::new();
    let nodes: Vec<_> = (0..50)
        .map(|i| graph.add_node(XffValue::from(format!("node_{}", i)), XffValue::Null))
        .collect();
    for i in 0..50 {
        let from = nodes[i];
        let to = nodes[(i + 1) % 50];
        let to_other = nodes[(i + 25) % 50];
        let _ = graph.add_connection(
            from,
            to,
            XffValue::from(format!("edge_{}_to_{}", i, (i + 1) % 50)),
        );
        let _ = graph.add_connection(
            from,
            to_other,
            XffValue::from(format!("edge_{}_to_{}", i, (i + 25) % 50)),
        );
    }
    let val_graph = XffValue::Graph(graph);
    let buf_graph =
        serialize_xff_to_buffer(vec![val_graph.clone()], 4).expect("Failed serialization");

    run_bench(
        "graph",
        "Ser",
        500,
        || serialize_xff_to_buffer(vec![val_graph.clone()], 4).unwrap(),
        Some(buf_graph.len()),
    )
    .print();

    run_bench(
        "graph",
        "Deser",
        500,
        || deserialize_xff_from_buffer(&buf_graph).unwrap(),
        Some(buf_graph.len()),
    )
    .print();

    // --- Profiling / Isolation Cases ---
    let raw_profile_data = vec![127u8; 1024 * 1024];

    // 1. CPU-Only: CRC32 computation on 1 MB
    run_bench(
        "profile: crc32_1mb_only",
        "CPU",
        100,
        || athena::checksum::crc32(&raw_profile_data),
        Some(raw_profile_data.len()),
    )
    .print();

    // 2. MEM-Only: Temp allocation & copying of 1 MB
    run_bench(
        "profile: alloc_and_copy_1mb",
        "MEM",
        100,
        || {
            let mut payload = Vec::with_capacity(raw_profile_data.len() + 8);
            payload.extend_from_slice(&raw_profile_data);
            payload
        },
        Some(raw_profile_data.len()),
    )
    .print();

    // 3. Opt: In-place serialization of 1 MB (eliminates intermediate allocation/copy)
    run_bench(
        "profile: in_place_ser_1mb",
        "Opt",
        100,
        || {
            let len_bytes = athena::encoding_and_decoding::serialize_leb128_unsigned(
                raw_profile_data.len() as u128,
            );
            let mut buf = Vec::with_capacity(6 + len_bytes.len() + raw_profile_data.len());
            buf.push(0x21); // complex::DAT
            let payload_start = buf.len();
            buf.extend_from_slice(&len_bytes);
            buf.extend_from_slice(&raw_profile_data);
            let payload_end = buf.len();
            let checksum = athena::checksum::crc32(&buf[payload_start..payload_end]);
            buf.extend_from_slice(&checksum.to_le_bytes());
            buf.push(0x60); // internal::EV
            buf
        },
        Some(raw_profile_data.len() + 6),
    )
    .print();
}
