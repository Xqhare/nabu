mod error;

pub mod xff;

const XFF_VERSION: u8 = 0;

pub mod serde {
    use crate::error::NabuError;
    use crate::xff::deserializer::deserialize_xff;
    use crate::xff::serializer::serialize_xff;
    use crate::xff::value::XffValue;
    use crate::XFF_VERSION;

    pub fn read<P>(path: P) -> Result<Vec<XffValue>, NabuError> where P: AsRef<std::path::Path> {
        let path_with_xff_extension = path.as_ref().with_extension("xff");
        deserialize_xff(&path_with_xff_extension)
    }

    pub fn write<P>(path: P, data: Vec<XffValue>) -> Result<(), NabuError> where P: AsRef<std::path::Path> {
        let path_with_xff_extension = path.as_ref().with_extension("xff");
        serialize_xff(&path_with_xff_extension, data, XFF_VERSION)
    }

    pub fn write_legacy<P>(path: P, data: Vec<XffValue>, xff_version: u8) -> Result<(), NabuError> where P: AsRef<std::path::Path> {
        let path_with_xff_extension = path.as_ref().with_extension("xff");
        serialize_xff(&path_with_xff_extension, data, xff_version)
    }
}



// I have two possible architechures, one simply reading and writing a hashmap or btree, and one
// that creates a structure holding and storing the data for interaction that is also capable of serializing.
//
// I have chosen both, split up in two features, one containing the core functions (key_value_core == the first architecture) and one containing the store functions (key_value_store == the second architecture).

//#[cfg(feature = "key_value_core")]
pub mod key_value_core {
    use std::collections::BTreeMap;

    use crate::{error::NabuError, xff::value::XffValue};

    pub fn read<P, E>(path: P) -> Result<BTreeMap<String, XffValue>, E> where P: AsRef<std::path::Path>, E: Into<NabuError> {
        
    }
}

#[cfg(feature = "key_value_store")]
pub mod key_value_store {
    
}
