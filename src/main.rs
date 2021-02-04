mod common;
mod config_parse;
mod hue;
mod nanoleaf;

use clap::{App, ArgMatches};
use common::{Lamp, Sig};
use config_parse::Config;
use hue::Hue;
use nanoleaf::Nanoleaf;
use std::thread;

#[macro_use]
extern crate clap;

fn main() {
    let config_path = common::get_config_path();
    let yaml = load_yaml!("cli.yaml");
    let arg_parse = App::from_yaml(yaml).get_matches();

    // Declare all the lights
    let lights: Vec<Box<dyn Lamp + Send>> = vec![
        Box::new(Nanoleaf::new(&config_path)),
        Box::new(Hue::new(&config_path, 1)),
        Box::new(Hue::new(&config_path, 2)),
    ];

    // Select the lights of intrest as given by the CLI arguments
    let selected_lamp_id: Vec<usize> = values_t!(arg_parse.values_of("lamp"), usize).unwrap();
    let filtered_lights = lights
        .into_iter()
        .enumerate()
        .filter(move |(num, _)| selected_lamp_id.contains(num));

    // Dispatch each command to a new thread
    let config = get_config(&arg_parse);
    let mut threads = vec![];
    for (_, light) in filtered_lights {
        threads.push(thread::spawn(move || match config {
            Config::Gradient(args) => config_parse::set_gradient(args, light),
            Config::On(signal) => light.put(signal),
            Config::Off => light.put(Sig::On(false)),
        }));
    }

    // Ensure that all these threads have completed
    for t in threads {
        t.join().expect("Unable to complete thread");
    }
}

/// Get a configuration object from the command line arguments
fn get_config(args: &ArgMatches) -> Config {
    match &args.subcommand() {
        ("gradient", Some(args)) => config_parse::get_gradient_config(args),
        ("on", Some(args)) => config_parse::get_on_config(args),
        ("off", _) => Config::Off,
        _ => Config::Off,
    }
}
