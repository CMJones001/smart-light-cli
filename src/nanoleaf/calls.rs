#![allow(dead_code)]
use serde::Serialize;
use std::collections::HashMap;

pub enum Action {
    PUT,
    POST,
}

pub trait Send {
    fn to_json(&self) -> String;
    fn url_ext(&self) -> &str;
    fn action(&self) -> Action;
}

#[derive(Serialize, Debug)]
pub struct NanoleafOn {
    on: HashMap<String, bool>,
}

impl NanoleafOn {
    pub fn new(status: bool) -> NanoleafOn {
        let mut map = HashMap::new();
        map.insert("value".to_string(), status);
        NanoleafOn { on: map }
    }
}
impl Send for NanoleafOn {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn url_ext(&self) -> &str {
        &"state"
    }

    fn action(&self) -> Action {
        Action::PUT
    }
}

#[derive(Serialize, Debug)]
pub struct NanoleafBrightness {
    brightness: HashMap<String, usize>,
}

impl NanoleafBrightness {
    pub fn new(value: usize, duration: Option<usize>) -> NanoleafBrightness {
        let mut map = HashMap::new();
        map.insert("value".to_string(), value);
        if let Some(dur) = duration {
            map.insert("duration".to_string(), dur);
        }
        NanoleafBrightness { brightness: map }
    }
}
impl Send for NanoleafBrightness {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn url_ext(&self) -> &str {
        &"state"
    }

    fn action(&self) -> Action {
        Action::PUT
    }
}
