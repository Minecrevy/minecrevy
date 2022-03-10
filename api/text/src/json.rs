use std::io;

use crate::Text;

impl Text {
    pub fn from_json_string(str: &str) -> io::Result<Text> {
        Ok(serde_json::from_str(str)?)
    }

    pub fn to_json_string(&self) -> String {
        serde_json::to_string(&self).expect("shouldn't fail to serialize")
    }
}
