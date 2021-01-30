use crate::common::ApiCommand;
use crate::common::Lamp;
use ini::Ini;
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
        inner_struct.insert("bri", val);
        let json = serde_json::to_string(&inner_struct).unwrap();
        ApiCommand { addr, json }
    }

    fn colour_command(&self, hue: isize, sat: isize, bri: isize) -> ApiCommand {
        let addr = "state".to_string();
        let mut outer_struct = HashMap::new();

        let vals = vec![hue, sat, bri];
        let labels = vec!["hue", "sat", "bri"];
        for (val, label) in vals.iter().zip(labels) {
            outer_struct.insert(label, val);
        }

        let json = serde_json::to_string(&outer_struct).unwrap();
        ApiCommand { addr, json }
    }
}
