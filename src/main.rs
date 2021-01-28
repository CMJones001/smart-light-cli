mod nanoleaf;

use clap::{App, ArgMatches, ErrorKind::ArgumentNotFound};
use ini::Ini;
use nanoleaf::calls as nc;
use nanoleaf::calls::{Action, Get, Send};
use reqwest::blocking;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::path::PathBuf;

#[macro_use]
extern crate clap;

#[derive(Debug)]
struct Nanoleaf {
    ip: String,
    api_key: String,
}

/// API interface for the Nanoleaf lights
///
/// Commands can be sent by using the ``get`` method.
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
    fn get<S: Get>(&self, signal: &S) -> String {
        let full_request = format!("{addr}/{ext}", addr = self.addr(), ext = signal.get_url());

        let response = blocking::get(&full_request)
            .expect("Unable to send request")
            .text()
            .unwrap();

        println!("response = {:?}", response);
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
}

fn main() {
    let config_path = PathBuf::from("conf.ini");
    let light = Nanoleaf::new(&config_path);

    let yaml = load_yaml!("cli.yaml");
    let arg_parse = App::from_yaml(yaml).get_matches();

    match arg_parse.subcommand_name() {
        Some("on") => light.run(&nc::On::new(true)),
        Some("off") => light.run(&nc::On::new(false)),
        Some("brightness") => set_brightness(&arg_parse, light),
        _ => {
            println!("No command provided, give --help to see options");
            std::process::exit(1)
        }
    };
}

/// Set the brightness of the lights from command line arguments
fn set_brightness(arg_parse: &ArgMatches, light: Nanoleaf) {
    let brightness_args = arg_parse.subcommand_matches("brightness").unwrap();
    let val: isize = brightness_args
        .value_of("val")
        .unwrap()
        .parse()
        .expect("Unable to parse brightness value into int");

    // Return None if there is no duration provided. Quit if the provided value
    // cannot be parsed.
    let duration = if let Some(dur) = brightness_args.value_of("duration") {
        Some(dur.parse::<usize>().unwrap_or_else(|_| {
            println!("Unable to parse duration argument");
            std::process::exit(1);
        }))
    } else {
        // If nothing is provided
        None
    };

    let brightness = nc::Brightness::new(val, duration);
    light.run(&brightness);
}
