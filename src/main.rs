mod nanoleaf;

use ini::Ini;
use nanoleaf::calls::{self, Action, Send};
use reqwest::blocking;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::path::PathBuf;

#[derive(Debug)]
struct Nanoleaf {
    ip: String,
    api_key: String,
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

    fn run<S: Send>(&self, signal: &S) {
        match signal.action() {
            Action::PUT => self.put(signal),
            Action::POST => {}
        }
    }

    /// Send a PUT request to the nanoleaf
    ///
    /// # Arguments
    ///
    /// * `signal` - A nanoleaf call
    fn put<S: Send>(&self, signal: &S) {
        let request_url = format!("{addr}/{ext}", addr = self.addr(), ext = signal.url_ext());
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = blocking::Client::new();
        let res = client
            .put(&request_url)
            .headers(headers)
            .body(signal.to_json())
            .send()
            .expect("Unable to send PUT request");

        // TODO: add logging to catch if this fails
        let success = res.status().is_success();
        println!("success = {}", success);
    }
}

fn main() {
    let config_path = PathBuf::from("conf.ini");
    let light = Nanoleaf::new(&config_path);

    // let status_on = calls::NanoleafOn::new(true);
    // light.run(status_on)

    let brightness = calls::NanoleafBrightness::new(100, Some(5));
    light.run(&brightness);
    println!("brightness = {:?}", brightness);
}
