//! Generate API calls for the Phillips Hue lights
//!
//! # API overview
//!
//! The signal to change the HSV of a light bulb are given as single layer dictionary.
//! We note however, that if the light is off, it will not turn back on when the colour
//! is changed unless an "on" command is also sent explicitly.
//!
//! The values take a range of:
//! - hue: [0, 65535]
//! - sat: [0, 255]
//! - bri: [0, 255]
//! - on: true/false
//!
//! ## Example command
//!
//! ``{"hue":30000, "sat":200, "bri":255, "on":true}``
//!
//! ## Colour temperature
//!
//! There is a colour temperature independant of the colour commands
//! - ct: [154, 500]
//!
//! In this case the lower values are the colder temperature
//!
//! The bulbs are address independantly, via a ``lamp_id`` value.

use crate::common::{scale, scalegen, ApiCommand, Lamp};
use ini::Ini;
use palette::Hsv;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Controller for the Hue RGB lights.
///
/// # Parameters:
///
/// * `path` - Path to the .ini file containing a ``[hue]`` section with
///         the API key and IP address of the hue bridge.
/// * `lamp_id` - Index of the lamp.
pub struct Hue {
    ip: String,
    api_key: String,
    lamp_id: isize,
}

/// Container for the mixed data type values to be parsed into json
#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct ColourOnDict {
    hue: isize,
    bri: isize,
    sat: isize,
    on: bool,
}

#[derive(Serialize)]
struct TempOnDict {
    ct: isize,
    on: bool,
}

impl Hue {
    pub fn new(path: &PathBuf, lamp_id: isize) -> Hue {
        let conf = Ini::load_from_file(path).expect("unable to load .ini file");
        let section = conf
            .section(Some("hue"))
            .expect("Unable to find [hue] header");

        let ip = section
            .get("ip")
            .expect("IP address not found in hue .ini file");
        let api_key = section
            .get("api")
            .expect("API key not found in hue .ini file");

        Hue {
            ip: ip.to_string(),
            api_key: api_key.to_string(),
            lamp_id,
        }
    }
}

impl Lamp for Hue {
    fn addr(&self) -> String {
        format!(
            "http://{ip}/api/{api_key}/lights/{id}",
            ip = self.ip,
            api_key = self.api_key,
            id = self.lamp_id,
        )
    }

    fn on_command(&self, state: bool) -> Option<ApiCommand> {
        let addr = "state/on".to_string();
        let mut inner_struct = HashMap::new();
        inner_struct.insert("on", state);

        let json = serde_json::to_string(&inner_struct).unwrap();
        Some(ApiCommand { addr, json })
    }

    fn brightness_command(&self, val: isize) -> Option<ApiCommand> {
        let addr = "state".to_string();
        let mut inner_struct = HashMap::new();
        let val = scale(val, 100, 255);
        inner_struct.insert("bri", val);
        let json = serde_json::to_string(&inner_struct).unwrap();
        Some(ApiCommand { addr, json })
    }

    fn colour_command(&self, hue: isize, sat: isize, bri: isize) -> Option<ApiCommand> {
        let addr = "state".to_string();
        let hue = scale(hue, 360, 65535);
        let bri = scale(bri, 100, 255);
        let sat = scale(sat, 100, 255);
        let on = true;

        let colours = ColourOnDict { hue, bri, sat, on };
        let json = serde_json::to_string(&colours).unwrap();
        Some(ApiCommand { addr, json })
    }

    fn palette_command(&self, col: Hsv) -> Option<ApiCommand> {
        let addr = "state".to_string();
        // TODO: We have to change the dict type into a mixed dict for the on command

        let hue = scalegen(col.hue.to_positive_degrees(), 360.0, 65535);
        let sat = scalegen(col.saturation, 1.0, 255);
        let bri = scalegen(col.value, 1.0, 255);
        let on = true;

        let mixed_dict = ColourOnDict { hue, sat, bri, on };
        let json = serde_json::to_string(&mixed_dict).unwrap();
        Some(ApiCommand { addr, json })
    }

    fn temperature_command(&self, temp: isize) -> Option<ApiCommand> {
        let addr = "state".to_string();
        let temp_dict = TempOnDict::new(temp);
        let json = serde_json::to_string(&temp_dict).unwrap();
        Some(ApiCommand { addr, json })
    }
}

impl TempOnDict {
    fn new(value: isize) -> TempOnDict {
        let ct = temp_mapping(value);
        let on = true;
        TempOnDict { ct, on }
    }
}

fn temp_mapping(val: isize) -> isize {
    scale(100 - val, 100, 346) + 154
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case( 50.0 =>  50)]
    #[test_case( 0.0 =>  0)]
    #[test_case( 270.0 =>  270)]
    fn test_get_hue(hue: f64) -> isize {
        let colour = Hsv::new(hue, 0.5, 0.5);
        colour.hue.to_positive_degrees() as isize
    }

    #[test_case( 0.1 =>  25)]
    #[test_case( 0.0 =>  0)]
    #[test_case( 1.0 =>  255)]
    fn test_get_sat(sat: f64) -> isize {
        let colour = Hsv::new(10.0, sat, 0.5);
        scalegen(colour.saturation, 1.0, 255)
    }

    #[test_case(50.0, 0.1, 0.1)]
    #[test_case(180.0, 0.9, 0.9)]
    #[test_case(270.0, 1.0, 1.0)]
    fn test_parse_hsv(hue_un: f32, sat_un: f32, bri_un: f32) {
        let colour = Hsv::new(hue_un, sat_un, bri_un);
        let light = Hue {
            ip: "".to_string(),
            api_key: "".to_string(),
            lamp_id: 0,
        };
        let api = light.palette_command(colour).unwrap();
        let json_test = api.json;
        let colour_map_test: ColourOnDict = serde_json::from_str(&json_test).unwrap();

        let hue = (hue_un / 360.0 * 65535.0) as isize;
        let sat = (sat_un * 255.0) as isize;
        let bri = (bri_un * 255.0) as isize;
        let colour_map_expected = ColourOnDict {
            hue,
            sat,
            bri,
            on: true,
        };

        assert_eq!(colour_map_test, colour_map_expected);
    }

    #[test_case(100 => 154)]
    #[test_case(0 => 500)]
    #[test_case(50 => 327)]
    fn test_temp_mapping(val_in: isize) -> isize {
        temp_mapping(val_in)
    }
}
