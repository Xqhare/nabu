#[cfg(test)]
mod v2 {
    use std::collections::BTreeMap;
    use std::{fs, usize};

    use serde::{read, remove_file, write};
    use tyche::prelude::*;

    use nabu::*;

    #[test]
    fn hello_world() {
        let path = "xff-example-data/v2-hello-world.xff";

        let mut object: Object = Object::new();

        object.insert("String", XffValue::from("Hi mom!"));
        object.insert("Number", XffValue::from(usize::MAX));
        object.insert("Number", XffValue::from(-42));
        object.insert("Number", XffValue::from(42.69));
        object.insert("Boolean", XffValue::from(true));
        object.insert("Null", XffValue::from(XffValue::Null));

        let mut array: Array = Array::new();
        array.push(XffValue::from("Hello mom!"));
        array.push(XffValue::from(usize::MAX));

        object.insert("Array", XffValue::from(array));

        object.insert("Data", XffValue::from(Data::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])));

        let value = XffValue::from(object);

        let write = write(path, value.clone());
        assert!(write.is_ok());
        let read = read(path);
        println!("{:?}", read);
        assert!(read.is_ok());
        let ok = read.unwrap();
        assert_eq!(ok, value);
        let remove = remove_file(path);
        assert!(remove.is_ok());
    }
}
