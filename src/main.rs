mod colours;
mod common;
mod hue;
mod nanoleaf;

use clap::{App, ArgMatches};
use common::Light;
use hue::light::Hue;
use nanoleaf::light::Nanoleaf;

#[macro_use]
extern crate clap;

fn main() {
    let config_path = common::get_config_path();
    let nanoleaf = Nanoleaf::new(&config_path);
    let hue_livinigroom = Hue::new(&config_path, 1);
    let hue_bedroom = Hue::new(&config_path, 2);

    let light = nanoleaf;
    let yaml = load_yaml!("cli.yaml");
    let arg_parse = App::from_yaml(yaml).get_matches();

    match arg_parse.subcommand_name() {
        Some("off") => light.on(false),
        Some("on") => turn_on_light(&arg_parse, light),
        _ => {
            println!("No command provided, give --help to see options");
            std::process::exit(1)
        }
    };
}

/// Parse the command line argument for turning the lights on
fn turn_on_light<L: Light>(arg_parse: &ArgMatches, light: L) {
    let brightness_args = arg_parse.subcommand_matches("on").unwrap();

    if let Some(val) = brightness_args.value_of("val") {
        let brightness = val.parse().unwrap();
        light.brightness(brightness);
    } else if let Some(colour_args) = brightness_args.values_of("colour") {
        let c: Vec<isize> = colour_args
            .map(|i| i.parse().expect("Unable to parse colour arguments"))
            .collect();
        // This is how to unpack a vector in rust apparently
        if let [hue, sat, brightness] = c[..] {
            light.colour(hue, sat, brightness);
        }
    } else {
        light.on(true);
    }
}
