mod colours;
mod common;
mod config_parse;
mod hue;
mod nanoleaf;

use clap::{App, ArgMatches};
use common::{Lamp, Sig};
use config_parse::{Config, GradientArgs, OnArgs};
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

    let lights: Vec<Box<dyn Lamp + Send>> = vec![
        Box::new(Nanoleaf::new(&config_path)),
        Box::new(Hue::new(&config_path, 1)),
        Box::new(Hue::new(&config_path, 2)),
    ];

    let filtered_lights = lights
        .into_iter()
        .enumerate()
        .filter(move |(num, _)| lamp_id.contains(num));

    let config = get_config(&arg_parse);

    let mut threads = vec![];
    for (_, light) in filtered_lights {
        threads.push(thread::spawn(move || match config {
            Config::Gradient(args) => set_gradient(args, light),
            Config::On(args) => set_on(args, light),
            Config::Off => light.put(Sig::On(false)),
        }));
    }

    for t in threads {
        t.join();
    }

    // thread::spawn(|| {
    //     for light in filtered_light()
    // })
    // for (_light_id, light) in filtered_lights {
    //     match &arg_parse.subcommand() {
    //         ("on", Some(args)) => light.put(get_on_signal(&args)),
    //         ("off", Some(_args)) => light.put(Sig::On(false)),
    //         ("gradient", Some(args)) => set_gradient(args, light),
    //         _ => {}
    //     }
    // }
}

fn get_config(args: &ArgMatches) -> Config {
    match &args.subcommand() {
        ("gradient", Some(args)) => config_parse::get_gradient_config(args),
        ("on", Some(args)) => config_parse::get_on_config(args),
        ("off", _) => Config::Off,
        _ => Config::Off,
    }
}

/// Transition between two colours
fn set_gradient(args: GradientArgs, light: Box<dyn Lamp>) {
    let hue_one = Hsv::new(30.0, 1.0, 0.8);
    let hue_two = Hsv::new(30.0, 0.3, 0.8);

    let grad = Gradient::new(vec![hue_one, hue_two]);
    let delay = time::Duration::from_secs(args.total_time) / (args.n_steps as u32);

    for (i, colour) in grad.take(args.n_steps as usize).enumerate() {
        light.put(Sig::Palette(colour));
        thread::sleep(delay);
    }
}

fn set_on(args: OnArgs, light: Box<dyn Lamp>) {
    // It looks that we might be able to reduce this down and skip the conversion step
    let sig = match args {
        OnArgs::Palette(pal) => Sig::Palette(pal),
        OnArgs::Colour(hue, sat, bri) => Sig::Colour(hue, sat, bri),
        OnArgs::Brightness(bri) => Sig::Brightness(bri),
        OnArgs::On => Sig::On(true),
    };

    light.put(sig);
}
