mod colours;
mod common;
mod hue;
mod nanoleaf;

use clap::{App, ArgMatches};
use common::{Lamp, Sig};
use hue::Hue;
use nanoleaf::Nanoleaf;
use palette::Hsv;

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
    if let Some(on_args) = args.subcommand_matches("on") {
        // Parse the on sub group
        if let Some(val) = on_args.value_of("val") {
            let brightness = val.parse().unwrap();
            Sig::Brightness(brightness)
        } else if on_args.is_present("colour") {
            let c = values_t_or_exit!(on_args.values_of("colour"), isize);
            if let [hue, sat, brightness] = c[..] {
                Sig::Colour(hue, sat, brightness)
            } else {
                unreachable!()
            }
        } else if on_args.is_present("palette") {
            let p = values_t_or_exit!(on_args.values_of("palette"), f32);
            if let [hue, sat, brightness] = p[..] {
                let pal = Hsv::new(hue, sat, brightness);
                Sig::Palette(pal)
            } else {
                unreachable!()
            }
        } else {
            Sig::On(true)
        }
    } else {
        Sig::On(false)
    }
}
