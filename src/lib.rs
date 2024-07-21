mod xff;
mod error;

const XFF_VERSION: u8 = 0;

pub mod serde {
    use crate::error::NabuError;
    use crate::xff::deserializer::deserialize_xff;
    use crate::xff::serializer::serialize_xff;
    use crate::xff::value::XffValue;
    use crate::XFF_VERSION;

    pub fn read<P>(path: P) -> Result<Vec<XffValue>, NabuError> where P: AsRef<std::path::Path> {
        deserialize_xff(path.as_ref())
    }

    pub fn write<P>(path: P, data: Vec<XffValue>) -> Result<(), NabuError> where P: AsRef<std::path::Path> {
        serialize_xff(path.as_ref(), data, XFF_VERSION)
    }

    pub fn write_legacy<P>(path: P, data: Vec<XffValue>, xff_version: u8) -> Result<(), NabuError> where P: AsRef<std::path::Path> {
        serialize_xff(path.as_ref(), data, xff_version)
    }
}
