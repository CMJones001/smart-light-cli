use crate::common::{Lamp, Sig};
use clap::ArgMatches;
use core::str::FromStr;
use palette::{Gradient, Hsv};
use std::{thread, time};

/// Container for the command line arguments
///
/// The rest of the behaviour of the project follows from this enum.
#[derive(Clone)]
pub enum Config {
    Gradient(GradientArgs),
    On(Sig),
    Off,
    Scene(String, bool),
}

#[derive(Copy, Clone)]
pub struct GradientArgs {
    total_time: u64,
    n_steps: u64,
    hue_one: Hsv,
    hue_two: Hsv,
}

/// Parse the "gradient" subsection from the command line
pub fn get_gradient_config(args: &ArgMatches) -> Result<Config, &'static str> {
    let total_time = value_t_or_exit!(args, "time", u64);
    let n_steps = value_t_or_exit!(args, "steps", u64);
    // TODO: Parse the palettes from the command line arguments
    let hue_one = Hsv::new(30.0, 1.0, 1.0);
    let hue_two = Hsv::new(30.0, 0.3, 1.0);
    let args = GradientArgs {
        total_time,
        n_steps,
        hue_one,
        hue_two,
    };
    Ok(Config::Gradient(args))
}

/// Parse the "on" subsection from the command line
pub fn get_on_config(args: &ArgMatches) -> Result<Config, &'static str> {
    let conf = if let Some(bri) = args.value_of("brightness") {
        match bri.parse() {
            Ok(brightness) => Sig::Brightness(brightness),
            Err(_) => return Err("Unable to parse brightness values from CLI"),
        }
    } else if let Some(temp) = args.value_of("temperature") {
        match temp.parse() {
            Ok(temperature) => Sig::Temp(temperature),
            Err(_) => return Err("Unable to parse temperature values from CLI"),
        }
    } else if args.is_present("colour") {
        let (hue, sat, bri) = unpack_values::<isize>(args, "colour");
        Sig::Colour(hue, sat, bri)
    } else if args.is_present("palette") {
        let (hue, sat, bri) = unpack_values::<f32>(args, "palette");
        let pal = Hsv::new(hue, sat, bri);
        Sig::Palette(pal)
    } else {
        Sig::On(true)
    };
    Ok(Config::On(conf))
}

pub fn get_scene_config(args: &ArgMatches) -> Result<Config, &'static str> {
    if args.is_present("list") | !args.is_present("name") {
        return Ok(Config::Scene("".to_string(), true));
    }
    let scene_name = args.value_of("name").expect("Unable to parse scene name");
    Ok(Config::Scene(scene_name.to_string(), false))
}

/// Transition between two colours
pub fn set_gradient(args: &GradientArgs, light: Box<dyn Lamp>) {
    let grad = Gradient::new(vec![args.hue_one, args.hue_two]);
    let delay = time::Duration::from_secs(args.total_time) / (args.n_steps as u32);

    for (_, colour) in grad.take(args.n_steps as usize).enumerate() {
        light.put(&Sig::Palette(colour));
        thread::sleep(delay);
    }
}

fn unpack_values<T: FromStr + Copy>(args: &ArgMatches, label: &str) -> (T, T, T) {
    let c = values_t_or_exit!(args.values_of(label), T);
    if let [hue, sat, brightness] = c[..] {
        (hue, sat, brightness)
    } else {
        unreachable!()
    }
}
