use crate::common::{scale, ApiCommand, Lamp};
use ini::Ini;
use serde::Serialize;
use serde_json::{Map, Number, Value};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct Hue {
    ip: String,
    api_key: String,
    lamp_id: isize,
}

impl Hue {
    pub fn new(path: &PathBuf, lamp_id: isize) -> Hue {
        let conf = Ini::load_from_file(path).expect("unable to load .ini file");
        let section = conf
            .section(Some("hue"))
            .expect("Unable to find [hue] header");

        let ip = section
            .get("ip")
            .expect("IP address not found in hue .ini file");
        let api_key = section
            .get("api")
            .expect("API key not found in hue .ini file");

        Hue {
            ip: ip.to_string(),
            api_key: api_key.to_string(),
            lamp_id,
        }
    }
}

impl Lamp for Hue {
    fn addr(&self) -> String {
        format!(
            "http://{ip}/api/{api_key}/lights/{id}",
            ip = self.ip,
            api_key = self.api_key,
            id = self.lamp_id,
        )
    }

    fn on_command(&self, state: bool) -> ApiCommand {
        let addr = "state/on".to_string();
        let mut inner_struct = HashMap::new();
        inner_struct.insert("on", state);

        let json = serde_json::to_string(&inner_struct).unwrap();
        ApiCommand { addr, json }
    }

    fn brightness_command(&self, val: isize) -> ApiCommand {
        let addr = "state".to_string();
        let mut inner_struct = HashMap::new();
        let val = scale(val, 255, 100);
        inner_struct.insert("bri", val);
        let json = serde_json::to_string(&inner_struct).unwrap();
        ApiCommand { addr, json }
    }

    fn colour_command(&self, hue: isize, sat: isize, bri: isize) -> ApiCommand {
        let addr = "state".to_string();
        let mut mixed_dict = Map::new();

        // Note we have to use the map structure to allow for dicts with
        // mixed item types
        scale_add(&mut mixed_dict, "hue", hue, 65535, 360);
        scale_add(&mut mixed_dict, "sat", sat, 255, 100);
        scale_add(&mut mixed_dict, "bri", bri, 255, 100);

        mixed_dict.insert("on".to_string(), Value::Bool(true));

        let json = serde_json::to_string(&mixed_dict).unwrap();
        ApiCommand { addr, json }
    }
}

fn scale_add(
    map: &mut serde_json::Map<String, Value>,
    label: &str,
    val: isize,
    max_val: isize,
    old_max: isize,
) {
    map.insert(
        label.to_string(),
        Value::Number(Number::from(scale(val, max_val, old_max))),
    );
}
