//! Light implmentation for the nanoleaf
//!
//! Implements the Light trait allowing for turning the light and off
//! and colour setting

use super::calls::{self};
use crate::common::{Action, Light, Send};
use ini::Ini;
use reqwest::blocking;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::path::PathBuf;

/// API interface for the Nanoleaf lights
///
/// Commands can be sent by using the ``get`` method.
#[derive(Debug)]
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

impl Light for Nanoleaf {
    fn on(&self, status: bool) {
        let on_sig = calls::On::new(status);
        &self.put(&on_sig);
    }

    fn colour(&self, hue: isize, sat: isize, bri: isize) {
        let col_sig = calls::Colour::new(hue, sat, bri);
        &self.put(&col_sig);
    }

    fn brightness(&self, val: isize) {
        let bright_sig = calls::Brightness::new(val, None);
        &self.put(&bright_sig);
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
}
