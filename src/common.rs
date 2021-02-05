//! Tools common to any light struct
use num_traits::{cast, NumCast, NumOps};
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
#[derive(Clone)]
pub enum Sig {
    /// Turn the lamp on or off
    On(bool),
    /// Set the brightness of the lamp. Values from [0, 100] are accepted
    Brightness(isize),
    /// Set the colour to the (hue, saturation, brightness) tuple.
    /// The maximum values are (360, 100, 100).
    Colour(isize, isize, isize),
    /// Set the colour using the Hsv struct.
    /// This has better gradient interpolation between colours.
    Palette(Hsv),
    Temp(isize),
    Scene(String),
}

pub enum GetSig {
    Scene,
}

#[derive(Debug)]
pub struct ApiCommand {
    pub addr: String,
    pub json: String,
}

/// Documentation for the Lamp struct
pub trait Lamp {
    fn on(&self, state: bool) {
        self.put(&Sig::On(state));
    }

    fn brightness(&self, val: isize) {
        self.put(&Sig::Brightness(val));
    }

    fn colour(&self, hue: isize, sat: isize, bri: isize) {
        self.put(&Sig::Colour(hue, sat, bri))
    }

    fn palette(&self, pal: Hsv) {
        self.put(&Sig::Palette(pal))
    }

    fn temperature(&self, temp: isize) {
        self.put(&Sig::Temp(temp))
    }

    /// Send the given signal to the lamp via PUT request
    fn put(&self, signal: &Sig) {
        let cmd_signal = match signal {
            Sig::On(state) => self.on_command(*state),
            Sig::Brightness(val) => self.brightness_command(*val),
            Sig::Colour(hue, sat, bri) => self.colour_command(*hue, *sat, *bri),
            Sig::Palette(col) => self.palette_command(*col),
            Sig::Temp(temp) => self.temperature_command(*temp),
            Sig::Scene(name) => self.scene_command(name.to_string()),
        };

        let cmd = match cmd_signal {
            Some(c) => c,
            None => return,
        };

        let request_url = format!("{addr}/{ext}", addr = self.addr(), ext = cmd.addr);
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        if cfg!(debug_assertions) {
            println!("signal = {:#?}", cmd.json);
        }

        let client = blocking::Client::new();
        let res = client
            .put(&request_url)
            .headers(headers)
            .body(cmd.json)
            .send()
            .expect("Unable to send PUT request");

        // TODO: add logging to catch if this fails
        let success = res.status().is_success();
        if !success {
            println!("success = {}", success);
        }
    }

    fn get(&self, signal: GetSig) -> String {
        let cmd = match signal {
            GetSig::Scene => ApiCommand {
                addr: "effects/effectsList".to_string(),
                json: "".to_string(),
            },
        };

        let request_url = format!("{addr}/{ext}", addr = self.addr(), ext = cmd.addr);
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        if cfg!(debug_assertions) {
            println!("signal = {:#?}", cmd.json);
        }

        let client = blocking::Client::new();
        let res = client
            .get(&request_url)
            .send()
            .expect("Unable to send GET request");

        // TODO: add logging to catch if this fails
        let success = res.status().is_success();
        if !success {
            println!("success = {}", success);
        }
        res.text().expect("unable to read GET text")
    }

    /// The base address for where to send the API requests
    fn addr(&self) -> String;
    /// Generate the request to turn the lamp on or off
    fn on_command(&self, _state: bool) -> Option<ApiCommand> {
        None
    }
    /// Generate the request to turn the lamp on or off
    fn brightness_command(&self, _val: isize) -> Option<ApiCommand> {
        None
    }
    /// Generate the request to change the colour of the lamp
    fn colour_command(&self, _hue: isize, _sat: isize, _bri: isize) -> Option<ApiCommand> {
        None
    }
    fn palette_command(&self, _colour: Hsv) -> Option<ApiCommand> {
        None
    }
    fn temperature_command(&self, _temp: isize) -> Option<ApiCommand> {
        None
    }
    fn scene_command(&self, _scene_name: String) -> Option<ApiCommand> {
        None
    }
}

/// Scale an interger value relative to one range into a new range
pub fn scale(input_val: isize, old_max: isize, new_max: isize) -> isize {
    input_val * new_max / old_max
}

pub fn scalegen<T>(input_val: T, old_max: T, new_max: isize) -> isize
where
    T: NumOps + NumCast,
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
    fn test_scalegen_hundred(val_unscaled: f64) -> isize {
        scalegen(val_unscaled, 1.0, 100)
    }

    #[test_case(0.50, 1.0, 360 => 180)]
    #[test_case(1.00, 1.0, 360 => 360)]
    #[test_case(0.50, 2.0, 360 => 90)]
    #[test_case(10.0, 100.0, 360 => 36)]
    fn test_scalegen_gen(val: f64, old_range: f64, new_range: isize) -> isize {
        scalegen(val, old_range, new_range)
    }

    #[test_case(0.50, 1.0, 360 => 180)]
    #[test_case(1.00, 1.0, 360 => 360)]
    #[test_case(0.50, 2.0, 360 => 90)]
    #[test_case(10.0, 100.0, 360 => 36)]
    fn test_scalegen_f32(val: f32, old_range: f32, new_range: isize) -> isize {
        scalegen(val, old_range, new_range)
    }
}
