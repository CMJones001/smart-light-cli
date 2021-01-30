use crate::common::Send;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct On {
    on: HashMap<String, bool>,
}

impl On {
    pub fn new(status: bool) -> On {
        let mut map = HashMap::new();
        map.insert("on".to_string(), status);

        On { on: map }
    }
}

impl Send for On {
    fn to_json(&self) -> String {
        serde_json::to_string(&self.on).unwrap()
    }
}
