use reqwest::blocking;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::path::PathBuf;
use xdg;

pub fn get_config_path() -> PathBuf {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("nanoleaf.cli").unwrap();
    xdg_dirs
        .place_config_file("conf.ini")
        .expect("Unable to place config file")
}

pub enum Sig {
    On(bool),
    Brightness(isize),
    Colour(isize, isize, isize),
}

#[derive(Debug)]
pub struct ApiCommand {
    pub addr: String,
    pub json: String,
}

pub trait Lamp {
    fn on(&self, state: bool) {
        self.put(Sig::On(state));
    }

    fn brightness(&self, val: isize) {
        self.put(Sig::Brightness(val));
    }

    fn colour(&self, hue: isize, sat: isize, bri: isize) {
        self.put(Sig::Colour(hue, sat, bri))
    }

    fn put(&self, signal: Sig) {
        let cmd = match signal {
            Sig::On(state) => self.on_command(state),
            Sig::Brightness(val) => self.brightness_command(val),
            Sig::Colour(hue, sat, bri) => self.colour_command(hue, sat, bri),
        };

        let request_url = format!("{addr}/{ext}", addr = self.addr(), ext = cmd.addr);
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = blocking::Client::new();
        let res = client
            .put(&request_url)
            .headers(headers)
            .body(cmd.json)
            .send()
            .expect("Unable to send PUT request");

        // TODO: add logging to catch if this fails
        let success = res.status().is_success();
        println!("success = {}", success);
    }

    fn addr(&self) -> String;
    fn on_command(&self, state: bool) -> ApiCommand;
    fn brightness_command(&self, val: isize) -> ApiCommand;
    fn colour_command(&self, hue: isize, sat: isize, bri: isize) -> ApiCommand;
}
