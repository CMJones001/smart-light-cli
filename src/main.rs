use ini::Ini;
use reqwest::blocking;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
struct Nanoleaf {
    ip: String,
    api_key: String,
}

#[derive(Serialize)]
struct NanoleafOn {
    on: HashMap<String, bool>,
}

impl Nanoleaf {
    pub fn new(path: &PathBuf) -> Nanoleaf {
        let conf = Ini::load_from_file(path).expect("unable to load .ini file");
        let section = conf
            .section(Some("api"))
            .expect("Unable to find [api] header");

        let ip = section
            .get("ip")
            .expect("IP address not found in .ini file");
        let api_key = section.get("api").expect("API key not found in .ini file");

        Nanoleaf {
            ip: ip.to_string(),
            api_key: api_key.to_string(),
        }
    }

    fn addr(&self) -> String {
        let port = 16021;
        format!(
            "http://{ip}:{port}/api/v1/{api_key}",
            ip = self.ip,
            port = port,
            api_key = self.api_key
        )
    }

    /// Send a GET request to the nanoleaf
    // TODO: We need to catch the first "/" in the ext
    fn get(&self, ext: &str) -> String {
        let full_request = format!("{addr}/{ext}", addr = self.addr(), ext = ext);

        let response = blocking::get(&full_request)
            .expect("Unable to send request")
            .text()
            .unwrap();

        response
    }

    /// Send a PUT request to the nanoleaf
    ///
    /// # Arguments
    ///
    /// * `ext` - The API extension
    /// * `data` - The data to send in the PUT request
    fn put(&self, ext: &str, data: &str) {
        let request_url = format!("{addr}/{ext}", addr = self.addr(), ext = ext);
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = blocking::Client::new();
        let res = client
            .put(&request_url)
            .headers(headers)
            .body(data.to_string())
            .send()
            .expect("Unable to send PUT request");

        // TODO: add logging to catch if this fails
        let success = res.status().is_success();
        println!("success = {}", success);
    }

    /// Turn the lights on or off
    fn on(&self, value: NanoleafOn) {
        let ext = "state";
        let data = value.to_json();
        self.put(&ext, &data)
    }
}

impl NanoleafOn {
    pub fn new(status: bool) -> NanoleafOn {
        let mut map = HashMap::new();
        map.insert("value".to_string(), status);
        NanoleafOn { on: map }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

fn main() {
    let config_path = PathBuf::from("conf.ini");
    let light = Nanoleaf::new(&config_path);

    // light.on(true)
    let status_on = NanoleafOn::new(true);
    light.on(status_on)
}
