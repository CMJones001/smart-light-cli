#![allow(dead_code)]
use serde::Serialize;
use std::collections::HashMap;

pub enum Action {
    PUT,
    POST,
}

pub trait Send {
    fn to_json(&self) -> String;
    fn url_ext(&self) -> &str {
        &"state"
    }
    fn action(&self) -> Action {
        Action::PUT
    }
}

#[derive(Serialize, Debug)]
pub struct On {
    on: HashMap<String, bool>,
}

impl On {
    pub fn new(status: bool) -> On {
        let mut map = HashMap::new();
        map.insert("value".to_string(), status);
        On { on: map }
    }
}
impl Send for On {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Serialize, Debug)]
pub struct Brightness {
    brightness: HashMap<String, usize>,
}
impl Brightness {
    pub fn new(value: usize, duration: Option<usize>) -> Brightness {
        let mut map = HashMap::new();
        map.insert("value".to_string(), value);
        if let Some(dur) = duration {
            map.insert("duration".to_string(), dur);
        }
        Brightness { brightness: map }
    }
}
impl Send for Brightness {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub struct Identify {}
impl Identify {
    pub fn new() -> Identify {
        Identify {}
    }
}
impl Send for Identify {
    fn to_json(&self) -> String {
        String::from("")
    }

    fn url_ext(&self) -> &str {
        &"identify"
    }
}
