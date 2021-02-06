mod common;
mod config_parse;
mod hue;
mod nanoleaf;

use crate::nanoleaf::SceneList;
use clap::{App, ArgMatches};
use common::{Lamp, Sig};
use config_parse::Config;
use hue::Hue;
use nanoleaf::Nanoleaf;
use std::sync::Arc;
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
    let selected_lamp_id: Vec<usize> = values_t!(arg_parse.values_of("lamp"), usize)
        .unwrap_or_else(|_| {
            println!("Unable to parse lamp IDs");
            std::process::exit(1)
        });
    let filtered_lights = lights
        .into_iter()
        .enumerate()
        .filter(move |(num, _)| selected_lamp_id.contains(num));

    let config = get_config(&arg_parse).unwrap_or_else(|e| {
        println!("Unable to parse config: {}", e);
        std::process::exit(1)
    });

    // Possibly quit early if the config > scene > list provided
    if let Config::Scene(_, list) = config {
        if list {
            let scenes = SceneList::new(&"/tmp/scene_list.json", None);
            println!("Scene list: {:?}", scenes.names);
            std::process::exit(0);
        }
    };

    // conf is moved to Arc that allows multiple references between threads
    let conf = Arc::new(config);
    let mut threads = vec![];
    for (_, light) in filtered_lights {
        let conf = Arc::clone(&conf);
        threads.push(thread::spawn(move || match conf.as_ref() {
            Config::Gradient(args) => config_parse::set_gradient(&args, light),
            Config::On(signal) => light.put(&signal),
            Config::Off => light.on(false),
            Config::Scene(name, _) => light.put(&Sig::Scene(name.clone())),
        }));
    }

    // Ensure that all these threads have completed
    for t in threads {
        t.join().expect("Unable to complete thread");
    }
}

/// Get a configuration object from the command line arguments
fn get_config(args: &ArgMatches) -> Result<Config, &'static str> {
    match &args.subcommand() {
        ("gradient", Some(args)) => config_parse::get_gradient_config(args),
        ("on", Some(args)) => config_parse::get_on_config(args),
        ("scene", Some(args)) => config_parse::get_scene_config(args),
        ("off", _) => Ok(Config::Off),
        _ => Err("No configuration values to read"),
    }
}
