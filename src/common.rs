use reqwest::blocking;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::path::PathBuf;
use xdg;

pub trait Light {
    fn on(&self, status: bool);
    fn colour(&self, hue: isize, sat: isize, bri: isize);
    fn brightness(&self, val: isize);

    fn addr(&self) -> String;
    fn put<S: Send>(&self, signal: &S) {
        let request_url = format!("{addr}/{ext}", addr = self.addr(), ext = signal.send_url());
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        println!("signal = {:#?}", signal);

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

    fn get<S: Get>(&self, signal: &S) -> String {
        let full_request = format!("{addr}/{ext}", addr = self.addr(), ext = signal.get_url());

        let response = blocking::get(&full_request)
            .expect("Unable to send request")
            .text()
            .unwrap();

        println!("response = {:?}", response);
        response
    }
}

pub trait Send: std::fmt::Debug {
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

pub enum Action {
    PUT,
    POST,
}

pub fn get_config_path() -> PathBuf {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("nanoleaf.cli").unwrap();
    xdg_dirs
        .place_config_file("conf.ini")
        .expect("Unable to place config file")
}
