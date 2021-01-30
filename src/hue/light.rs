use super::calls;
use crate::common::{Action, Light, Send};
use ini::Ini;
use std::path::PathBuf;

/// API interface for the Nanoleaf lights
#[derive(Debug)]
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
            .expect("Hue IP address not found in .ini file");
        let api_key = section
            .get("api")
            .expect("Hue API key not found in .ini file");

        Hue {
            ip: ip.to_string(),
            api_key: api_key.to_string(),
            lamp_id,
        }
    }
}

impl Light for Hue {
    fn on(&self, status: bool) {
        let on_sig = calls::On::new(status);
        &self.put(&on_sig);
    }

    fn brightness(&self, val: isize) {}
    fn colour(&self, hue: isize, sat: isize, bri: isize) {}

    fn addr(&self) -> String {
        format!(
            "http://{ip}/api/{apk}/lights/{id}",
            ip = self.ip,
            apk = self.api_key,
            id = self.lamp_id,
        )
    }
}
