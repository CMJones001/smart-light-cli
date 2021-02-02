//! Tools common to any light struct
use num_traits::{cast, Float, NumCast};
use palette::Hsv;
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

/// Possible signals to control the lamp
pub enum Sig {
    /// Turn the lamp on or off
    On(bool),
    /// Set the brightness of the lamp. Values from [0, 100] are accepted
    Brightness(isize),
    /// Set the colour to the (hue, saturation, brightness) tuple.
    /// The maximum values are (360, 100, 100).
    Colour(isize, isize, isize),
    Palette(Hsv),
}

#[derive(Debug)]
pub struct ApiCommand {
    pub addr: String,
    pub json: String,
}

/// Documentation for the Lamp struct
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

    fn palette(&self, pal: Hsv) {
        self.put(Sig::Palette(pal))
    }

    /// Send the given signal to the lamp via PUT request
    fn put(&self, signal: Sig) {
        let cmd = match signal {
            Sig::On(state) => self.on_command(state),
            Sig::Brightness(val) => self.brightness_command(val),
            Sig::Colour(hue, sat, bri) => self.colour_command(hue, sat, bri),
            Sig::Palette(col) => self.palette_command(col),
        };

        let request_url = format!("{addr}/{ext}", addr = self.addr(), ext = cmd.addr);
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        println!("signal = {:#?}", cmd.json);
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

    /// The base address for where to send the API requests
    fn addr(&self) -> String;
    /// Generate the request to turn the lamp on or off
    fn on_command(&self, state: bool) -> ApiCommand;
    /// Generate the request to turn the lamp on or off
    fn brightness_command(&self, val: isize) -> ApiCommand;
    /// Generate the request to change the colour of the lamp
    fn colour_command(&self, hue: isize, sat: isize, bri: isize) -> ApiCommand;
    fn palette_command(&self, colour: Hsv) -> ApiCommand;
}

pub fn scale(input_val: isize, old_max: isize, new_max: isize) -> isize {
    input_val * new_max / old_max
}

pub fn scaleftoi<T>(input_val: T, old_max: T, new_max: isize) -> isize
where
    T: Float + NumCast,
{
    let unit_val: T = input_val * cast(new_max).unwrap() / old_max;
    cast(unit_val).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case( 50 => 180 ; "Half range")]
    #[test_case( 100 => 360 ; "Full range")]
    #[test_case( 0 => 0 ; "Zero range")]
    fn test_simple_scale(val_unscaled: isize) -> isize {
        scale(val_unscaled, 100, 360)
    }

    #[test_case(0.50 => 50; "Half range")]
    #[test_case(0.1 => 10; "Full range")]
    #[test_case(0.0 => 0; "Zero range")]
    fn test_scaleftoi_hundred(val_unscaled: f64) -> isize {
        scaleftoi(val_unscaled, 1.0, 100)
    }

    #[test_case(0.50, 1.0, 360 => 180)]
    #[test_case(1.00, 1.0, 360 => 360)]
    #[test_case(0.50, 2.0, 360 => 90)]
    #[test_case(10.0, 100.0, 360 => 36)]
    fn test_scaleftoi_gen(val: f64, old_range: f64, new_range: isize) -> isize {
        scaleftoi(val, old_range, new_range)
    }

    #[test_case(0.50, 1.0, 360 => 180)]
    #[test_case(1.00, 1.0, 360 => 360)]
    #[test_case(0.50, 2.0, 360 => 90)]
    #[test_case(10.0, 100.0, 360 => 36)]
    fn test_scaleftoi_f32(val: f32, old_range: f32, new_range: isize) -> isize {
        scaleftoi(val, old_range, new_range)
    }
}
