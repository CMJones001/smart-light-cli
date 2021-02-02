mod colours;
mod common;
mod hue;
mod nanoleaf;

use clap::{App, ArgMatches};
use common::{Lamp, Sig};
use hue::Hue;
use nanoleaf::Nanoleaf;

#[macro_use]
extern crate clap;

fn main() {
    let config_path = common::get_config_path();
    let yaml = load_yaml!("cli.yaml");
    let arg_parse = App::from_yaml(yaml).get_matches();

    let lamp_id: Vec<usize> = values_t!(arg_parse.values_of("lamp"), usize).unwrap();
    let lights: Vec<Box<dyn Lamp>> = vec![
        Box::new(Nanoleaf::new(&config_path)),
        Box::new(Hue::new(&config_path, 1)),
        Box::new(Hue::new(&config_path, 2)),
    ];

    for id in lamp_id.iter() {
        let offset_id = id + 0;
        let light = &lights[offset_id];

        let sig = get_command_signal(&arg_parse);
        light.put(sig);
    }
}

fn get_command_signal(args: &ArgMatches) -> Sig {
    if let Some(brightness_args) = args.subcommand_matches("on") {
        // Parse the on sub group
        if let Some(val) = brightness_args.value_of("val") {
            let brightness = val.parse().unwrap();
            Sig::Brightness(brightness)
        } else if let Some(colour_args) = brightness_args.values_of("colour") {
            let c: Vec<isize> = colour_args
                .map(|i| i.parse().expect("Unable to parse colour arguments"))
                .collect();
            // This is how to unpack a vector in rust apparently
            if let [hue, sat, brightness] = c[..] {
                return Sig::Colour(hue, sat, brightness);
            }
            Sig::On(true)
        } else {
            Sig::On(true)
        }
    } else {
        Sig::On(false)
    }
}
