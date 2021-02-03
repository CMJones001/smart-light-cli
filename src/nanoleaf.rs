//! Generate API calls for nanoleaf
//!
//! # API Overview
//!
//! The signal to change the HSV are given as a nested dictionary, the key of the outer
//! level is the property to change, the value is the a dictionary with a key of "value"
//! and an integer with the requested value.
//!
//! The values take a range of:
//! - brightness: [0, 100]
//! - sat: [0, 100]
//! - hue: [0, 360]
//!
//! ## Example command
//!
//! ``{"brightness": {"value":70}, "sat": {"value":20}, "hue", {"value":120}}``
//!
//! We note that these command changes all panels.

use crate::common::{scalegen, ApiCommand, Lamp};
use ini::Ini;
use num_traits::{NumCast, NumOps};
use palette::Hsv;
use std::collections::HashMap;
use std::path::PathBuf;

type NestedDict = HashMap<String, HashMap<String, isize>>;

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

impl Lamp for Nanoleaf {
    fn addr(&self) -> String {
        let port = 16021;
        format!(
            "http://{ip}:{port}/api/v1/{api_key}",
            ip = self.ip,
            port = port,
            api_key = self.api_key
        )
    }

    fn on_command(&self, state: bool) -> ApiCommand {
        let addr = "state/on".to_string();
        let mut cmd_dict = HashMap::new();
        let mut inner_struct = HashMap::new();

        inner_struct.insert("value", state);
        cmd_dict.insert("on", inner_struct);

        let json = serde_json::to_string(&cmd_dict).unwrap();
        ApiCommand { addr, json }
    }

    fn brightness_command(&self, val: isize) -> ApiCommand {
        let addr = "state".to_string();
        let mut cmd_dict = HashMap::new();
        // TODO add duration keyword
        wrap_insert(&mut cmd_dict, "brightness", val);
        let json = serde_json::to_string(&cmd_dict).unwrap();
        ApiCommand { addr, json }
    }

    fn colour_command(&self, hue: isize, sat: isize, bri: isize) -> ApiCommand {
        let addr = "state".to_string();
        let mut cmd_dict = HashMap::new();

        wrap_insert(&mut cmd_dict, "hue", hue);
        wrap_insert(&mut cmd_dict, "sat", sat);
        wrap_insert(&mut cmd_dict, "brightness", bri);

        let json = serde_json::to_string(&cmd_dict).unwrap();
        ApiCommand { addr, json }
    }

    fn palette_command(&self, col: Hsv) -> ApiCommand {
        let addr = "state".to_string();
        let mut cmd_dict = HashMap::new();
        let hue = col.hue.to_positive_degrees();

        wrap_insert_scale(&mut cmd_dict, "hue", hue, 360.0, 360);
        wrap_insert_scale(&mut cmd_dict, "sat", col.saturation, 1.0, 100);
        wrap_insert_scale(&mut cmd_dict, "brightness", col.value, 1.0, 100);

        let json = serde_json::to_string(&cmd_dict).unwrap();
        ApiCommand { addr, json }
    }
}

/// Pack the hue, sat, bri values inside a nested dict
fn wrap_insert_scale<U: NumCast + NumOps>(
    outer_dict: &mut NestedDict,
    label: &str,
    value: U,
    old_range: U,
    new_range: isize,
) {
    let mut inner_struct = HashMap::new();
    let scaled_val = scalegen(value, old_range, new_range);
    inner_struct.insert("value".to_string(), scaled_val);
    outer_dict.insert(label.to_string(), inner_struct);
}

/// Pack the hue, sat, bri values inside a NestedDict without changing any values.
fn wrap_insert(outer_dict: &mut NestedDict, label: &str, value: isize) {
    let mut inner_struct = HashMap::new();
    inner_struct.insert("value".to_string(), value);
    outer_dict.insert(label.to_string(), inner_struct);
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_json_diff::assert_json_include;
    use test_case::test_case;

    #[test]
    fn test_simple_wrap() {
        let hue = 180;
        let sat = 100;
        let bri = 50;
        let mut dict_expected = HashMap::new();

        // Manually create the dict
        let vals = vec![hue, sat, bri];
        let labels = vec!["hue", "sat", "brightness"];
        for (val, label) in vals.iter().zip(labels) {
            let mut inner_struct = HashMap::new();
            inner_struct.insert("value".to_string(), *val);
            dict_expected.insert(label.to_string(), inner_struct);
        }

        // Wrap functions
        let mut dict_test = HashMap::new();
        wrap_insert(&mut dict_test, "hue", hue);
        wrap_insert(&mut dict_test, "sat", sat);
        wrap_insert(&mut dict_test, "brightness", bri);

        assert_eq!(dict_test, dict_expected)
    }

    #[test]
    fn test_scaled_wrap() {
        let hue = 180.0;
        let sat = 0.5;
        let bri = 1.0;
        let mut dict_expected = HashMap::new();

        // Manually create the dict
        let mut inner_struct = HashMap::new();
        let scaled_val = scalegen(hue, 360.0, 65535);
        inner_struct.insert("value".to_string(), scaled_val);
        dict_expected.insert("hue".to_string(), inner_struct);

        let mut inner_struct = HashMap::new();
        let scaled_val = scalegen(sat, 1.0, 100);
        inner_struct.insert("value".to_string(), scaled_val);
        dict_expected.insert("sat".to_string(), inner_struct);

        let mut inner_struct = HashMap::new();
        let scaled_val = scalegen(bri, 1.0, 100);
        inner_struct.insert("value".to_string(), scaled_val);
        dict_expected.insert("brightness".to_string(), inner_struct);

        // Wrap functions
        let mut dict_test = HashMap::new();
        wrap_insert_scale(&mut dict_test, "hue", hue, 360.0, 65535);
        wrap_insert_scale(&mut dict_test, "sat", sat, 1.0, 100);
        wrap_insert_scale(&mut dict_test, "brightness", bri, 1.0, 100);

        assert_eq!(dict_test, dict_expected)
    }

    #[test_case(10)]
    #[test_case(100)]
    fn test_brightness_command(val: isize) {
        let json_expected = format!(r#"{{"brightness":{{"value":{}}}}}"#, val);
        let test_lamp = Nanoleaf {
            ip: "".to_string(),
            api_key: "".to_string(),
        };

        let cmd = test_lamp.brightness_command(val);
        let json_test = cmd.json;

        assert_json_include!(actual: json_test, expected: json_expected)
    }

    // Test the correct generation of the colour json
    #[test_case(255, 100, 100)]
    #[test_case(80, 30, 20)]
    fn test_colour_command(hue: isize, sat: isize, bri: isize) {
        let json_expected = format!(
            r#"{{"brightness":{{"value":{}}},"sat":{{"value":{}}},"hue":{{"value":{}}}}}"#,
            bri, sat, hue
        );
        let result_expected: NestedDict = serde_json::from_str(&json_expected).unwrap();

        let test_lamp = Nanoleaf {
            ip: "".to_string(),
            api_key: "".to_string(),
        };
        let cmd = test_lamp.colour_command(hue, sat, bri);
        let result_test: NestedDict = serde_json::from_str(&cmd.json).unwrap();

        assert_eq!(result_test, result_expected)
    }
}