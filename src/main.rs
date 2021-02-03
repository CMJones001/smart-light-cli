mod colours;
mod common;
mod hue;
mod nanoleaf;

use clap::{App, ArgMatches};
use common::{Lamp, Sig};
use hue::Hue;
use nanoleaf::Nanoleaf;
use palette::{Gradient, Hsv};
use std::{thread, time};

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

        match &arg_parse.subcommand() {
            ("on", Some(args)) => light.put(get_on_signal(&args)),
            ("off", Some(_args)) => light.put(Sig::On(false)),
            ("gradient", Some(args)) => {
                println!("Gradient!");
            }
            _ => {}
        }
    }

    // let hue_one = Hsv::new(30.0, 1.0, 0.8);
    // let hue_two = Hsv::new(30.0, 0.3, 0.8);

    // let grad = Gradient::new(vec![hue_one, hue_two]);
    // let delay = time::Duration::from_secs(3);
    // let n_steps = 5;

    // for (i, colour) in grad.take(5).enumerate() {}
}

/// Parse command line arguments for the "on" group
///
/// This is used to set immediately set the colour of the light
fn get_on_signal(args: &ArgMatches) -> Sig {
    if let Some(val) = args.value_of("val") {
        let brightness = val.parse().unwrap();
        Sig::Brightness(brightness)
    } else if args.is_present("colour") {
        let c = values_t_or_exit!(args.values_of("colour"), isize);
        if let [hue, sat, brightness] = c[..] {
            Sig::Colour(hue, sat, brightness)
        } else {
            unreachable!()
        }
    } else if args.is_present("palette") {
        let p = values_t_or_exit!(args.values_of("palette"), f32);
        if let [hue, sat, brightness] = p[..] {
            let pal = Hsv::new(hue, sat, brightness);
            Sig::Palette(pal)
        } else {
            unreachable!()
        }
    } else {
        Sig::On(true)
    }
}

/// Unpack arguments into a hue sat brightness tuple
fn unpack_to_colour(args: &ArgMatches) {}
