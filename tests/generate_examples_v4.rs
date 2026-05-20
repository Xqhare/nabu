use athena::{LocalDate, LocalDateTime, LocalTime, XffValue};
use athena::float::HpFloat;
use athena::graph::Graph;
use nabu::serde::write_legacy;

#[test]
fn generate_v4_examples() {
    let base = "xff-example-data/";
    
    // Simple values
    write_legacy(format!("{}v4_null.xff", base), XffValue::Null, 4).unwrap();
    write_legacy(format!("{}v4_boolean_t.xff", base), XffValue::from(true), 4).unwrap();
    write_legacy(format!("{}v4_boolean_f.xff", base), XffValue::from(false), 4).unwrap();
    write_legacy(format!("{}v4_nan.xff", base), XffValue::NaN, 4).unwrap();
    write_legacy(format!("{}v4_pnan.xff", base), XffValue::PNan, 4).unwrap();
    write_legacy(format!("{}v4_nnan.xff", base), XffValue::NNan, 4).unwrap();
    write_legacy(format!("{}v4_infinity.xff", base), XffValue::Infinity, 4).unwrap();
    write_legacy(format!("{}v4_neginfinity.xff", base), XffValue::NegInfinity, 4).unwrap();

    // New complex types
    write_legacy(format!("{}v4_local_date.xff", base), XffValue::LocalDate(LocalDate::new(2026, 5, 16)), 4).unwrap();
    write_legacy(format!("{}v4_local_time.xff", base), XffValue::LocalTime(LocalTime::new(14, 30, 0, 0)), 4).unwrap();
    write_legacy(format!("{}v4_local_datetime.xff", base), XffValue::LocalDateTime(LocalDateTime::new(
        LocalDate::new(2026, 5, 16),
        LocalTime::new(14, 30, 0, 0)
    )), 4).unwrap();
    write_legacy(format!("{}v4_hp_float.xff", base), XffValue::HpFloat(HpFloat::new(123456789, 4)), 4).unwrap();
    write_legacy(format!("{}v4_ascii.xff", base), XffValue::Ascii(athena::XffString::from("ASCII text")), 4).unwrap();

    // Parent
    let mut g = Graph::new();
    let n1 = g.add_node(XffValue::from("Node 1"), XffValue::Null);
    let n2 = g.add_node(XffValue::from("Node 2"), XffValue::from("meta"));
    g.add_connection(n1, n2, XffValue::from("connection")).unwrap();
    write_legacy(format!("{}v4_graph.xff", base), XffValue::Graph(g), 4).unwrap();

    // Array with many elements for delta encoding example
    let ary = XffValue::from(vec![
        XffValue::from(1),
        XffValue::from(2),
        XffValue::from(3),
        XffValue::from("string"),
        XffValue::from(true),
    ]);
    write_legacy(format!("{}v4_array_delta.xff", base), ary, 4).unwrap();
}
