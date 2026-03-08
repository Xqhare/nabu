use nabu::XffValue;
use nabu::serde::{read, write, remove_file};
use athena::Metadata;

#[test]
fn test_metadata_basic() {
    let mut meta = Metadata::new();
    meta.set_creator("Xqhare".to_string());
    meta.set_description("Test File".to_string());
    meta.set_license("MIT".to_string());
    meta.set_created_at(123456789);

    assert_eq!(meta.get_creator(), Some("Xqhare".to_string()));
    assert_eq!(meta.get_description(), Some("Test File".to_string()));
    assert_eq!(meta.get_license(), Some("MIT".to_string()));
    assert_eq!(meta.get_created_at(), Some(123456789));
}

#[test]
fn test_v3_roundtrip_with_metadata() {
    let path = "test_meta_roundtrip.xff";
    
    let mut meta = Metadata::new();
    meta.set_creator("Gemini".to_string());
    
    let body = XffValue::from("The Body Content");
    
    // In our new API, if the first value is Metadata, it's moved to the head
    let data = vec![XffValue::Metadata(meta.clone()), body.clone()];
    
    write(path, data).unwrap();
    
    let read_val = read(path).unwrap();
    
    // Deserializer returns [Metadata, Body] if head metadata was present
    if let XffValue::Array(ary) = read_val {
        assert_eq!(ary.len(), 2);
        assert!(ary[0].is_metadata());
        assert_eq!(ary[1], body);
        
        let read_meta = ary[0].as_metadata().expect("Should be metadata");
        assert_eq!(read_meta.get_creator(), Some("Gemini".to_string()));
    } else {
        panic!("Read value should be an array containing [Metadata, Body]");
    }

    remove_file(path).unwrap();
}

#[test]
fn test_v3_roundtrip_no_metadata() {
    let path = "test_no_meta.xff";
    let body = XffValue::from("Just the body");
    
    write(path, body.clone()).unwrap();
    
    let read_val = read(path).unwrap();
    assert_eq!(read_val, body); // No metadata, so just returns the body

    remove_file(path).unwrap();
}
