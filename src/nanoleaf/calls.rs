#![allow(dead_code)]
use serde::Serialize;
use std::collections::HashMap;

pub enum Action {
    PUT,
    POST,
}

pub trait Send {
    fn to_json(&self) -> String;
    fn send_url(&self) -> &str {
        &"state"
    }
    fn action(&self) -> Action {
        Action::PUT
    }
}

pub trait Get {
    fn get_url(&self) -> &str;
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
impl Get for On {
    fn get_url(&self) -> &str {
        &"state/on"
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
        if value > 100 || value < 1 {
            panic!("Brightness must be in the range [0, 100]")
        }
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

#[derive(Serialize, Debug)]
pub struct Temperature {
    ct: HashMap<String, usize>,
}
impl Temperature {
    pub fn new(value: usize) -> Temperature {
        let mut map = HashMap::new();
        if value > 6500 || value < 1200 {
            panic!("Temperature must be in the range [1200, 6500]")
        }
        map.insert("value".to_string(), value);
        Temperature { ct: map }
    }
}
impl Send for Temperature {
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

    fn send_url(&self) -> &str {
        &"identify"
    }
}
