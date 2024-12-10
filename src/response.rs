use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use serde_json::Value;

// a helper struct to make it easy to construct json responses.
// Value is serde_json::Value, and can be any arbitrary json value.
pub enum Response {
    Ok(Value),
    Error(Value),
}

// custom impl to change field names based on the enum variant.
// could just use #[Serialize].
impl Serialize for Response {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Response", 2).unwrap();
        match self {
            Self::Ok(msg) => {
                s.serialize_field("success", &true).unwrap();
                s.serialize_field("data", msg).unwrap();
            }
            Self::Error(msg) => {
                s.serialize_field("success", &false).unwrap();
                s.serialize_field("error", msg).unwrap();
            }
        }

        s.end()
    }
}
