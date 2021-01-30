use crate::common::ApiCommand;
use crate::common::Lamp;
use ini::Ini;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct Nanoleaf {
    ip: String,
    api_key: String,
}

impl Nanoleaf {
    pub fn new(path: &PathBuf) -> Nanoleaf {
        let conf = Ini::load_from_file(path).expect("unable to load .ini file");
        let section = conf
            .section(Some("nanoleaf"))
            .expect("Unable to find [nanoleaf] header");

        let ip = section
            .get("ip")
            .expect("IP address not found in .ini file");
        let api_key = section.get("api").expect("API key not found in .ini file");

        Nanoleaf {
            ip: ip.to_string(),
            api_key: api_key.to_string(),
        }
    }
}

impl Lamp for Nanoleaf {
    fn addr(&self) -> String {
        let port = 16021;
        format!(
            "http://{ip}:{port}/api/v1/{api_key}",
            ip = self.ip,
            port = port,
            api_key = self.api_key
        )
    }

    fn on_command(&self, state: bool) -> ApiCommand {
        let addr = "state/on".to_string();
        let mut outer_struct = HashMap::new();
        let mut inner_struct = HashMap::new();

        inner_struct.insert("value", state);
        outer_struct.insert("on", inner_struct);

        let json = serde_json::to_string(&outer_struct).unwrap();
        ApiCommand { addr, json }
    }

    fn brightness_command(&self, val: isize) -> ApiCommand {
        let addr = "state".to_string();
        let mut outer_struct = HashMap::new();
        let mut inner_struct = HashMap::new();

        // TODO add duration keyword
        inner_struct.insert("value", val);
        outer_struct.insert("brightness", inner_struct);

        let json = serde_json::to_string(&outer_struct).unwrap();
        ApiCommand { addr, json }
    }

    fn colour_command(&self, hue: isize, sat: isize, bri: isize) -> ApiCommand {
        let addr = "state".to_string();
        let mut outer_struct = HashMap::new();

        let vals = vec![hue, sat, bri];
        let labels = vec!["hue", "sat", "brightness"];
        for (val, label) in vals.iter().zip(labels) {
            let mut inner_struct = HashMap::new();
            inner_struct.insert("value", val);
            outer_struct.insert(label, inner_struct);
        }

        let json = serde_json::to_string(&outer_struct).unwrap();
        ApiCommand { addr, json }
    }
}
