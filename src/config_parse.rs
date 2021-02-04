use crate::common::{Lamp, Sig};
use clap::ArgMatches;
use core::str::FromStr;
use palette::{Gradient, Hsv};
use std::{thread, time};

/// Container for the command line arguments
///
/// The rest of the behaviour of the project follows from this enum.
#[derive(Copy, Clone)]
pub enum Config {
    Gradient(GradientArgs),
    On(Sig),
    Off,
}

#[derive(Copy, Clone)]
pub struct GradientArgs {
    total_time: u64,
    n_steps: u64,
    hue_one: Hsv,
    hue_two: Hsv,
}

/// Parse the "gradient" subsection from the command line
pub fn get_gradient_config(args: &ArgMatches) -> Config {
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
    Config::Gradient(args)
}

/// Parse the "on" subsection from the command line
pub fn get_on_config(args: &ArgMatches) -> Config {
    let conf = if let Some(bri) = args.value_of("brightness") {
        let brightness = bri.parse().unwrap();
        Sig::Brightness(brightness)
    } else if let Some(temp) = args.value_of("temperature") {
        let temperature = temp.parse().unwrap();
        Sig::Temp(temperature)
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
    Config::On(conf)
}

/// Transition between two colours
pub fn set_gradient(args: GradientArgs, light: Box<dyn Lamp>) {
    let grad = Gradient::new(vec![args.hue_one, args.hue_two]);
    let delay = time::Duration::from_secs(args.total_time) / (args.n_steps as u32);

    for (_, colour) in grad.take(args.n_steps as usize).enumerate() {
        light.put(Sig::Palette(colour));
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
