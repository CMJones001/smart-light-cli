use clap::{App, ArgMatches};
use palette::{Gradient, Hsv};

#[derive(Copy, Clone)]
pub enum Config {
    Gradient(GradientArgs),
    On(OnArgs),
    Off,
}

#[derive(Copy, Clone)]
pub struct GradientArgs {
    pub total_time: u64,
    pub n_steps: u64,
}

#[derive(Copy, Clone)]
pub enum OnArgs {
    Palette(Hsv),
    Colour(isize, isize, isize),
    Brightness(isize),
    On,
}

pub fn get_gradient_config(args: &ArgMatches) -> Config {
    let total_time = args
        .value_of("time")
        .unwrap()
        .parse()
        .expect("Unable to parse gradient time");
    let n_steps = args
        .value_of("steps")
        .unwrap()
        .parse()
        .expect("Unable to parse gradient steps");
    let args = GradientArgs {
        total_time,
        n_steps,
    };
    Config::Gradient(args)
}

pub fn get_on_config(args: &ArgMatches) -> Config {
    let conf = if let Some(bri) = args.value_of("val") {
        let brightness = bri.parse().unwrap();
        OnArgs::Brightness(brightness)
    } else if args.is_present("colour") {
        let c = values_t_or_exit!(args.values_of("colour"), isize);
        if let [hue, sat, brightness] = c[..] {
            OnArgs::Colour(hue, sat, brightness)
        } else {
            unreachable!()
        }
    } else if args.is_present("palette") {
        let p = values_t_or_exit!(args.values_of("palette"), f32);
        if let [hue, sat, brightness] = p[..] {
            let pal = Hsv::new(hue, sat, brightness);
            OnArgs::Palette(pal)
        } else {
            unreachable!()
        }
    } else {
        OnArgs::On
    };
    Config::On(conf)
}
