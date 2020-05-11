extern crate clap;
#[macro_use]
extern crate fstrings;

use ::std::process::Command;

use clap::{App, load_yaml};

mod arg_validation;

fn get_display_names() -> Vec<String> {
    let xrandr_active = Command::new("xrandr")
        .arg("--listactivemonitors")
        .output().expect("Command xrandr failed")
        .stdout;
    let xrandr_active_str = String::from_utf8(xrandr_active).expect("Not UTF-8");

    // First row of xrandr listing is removed because it contains number of monitors
    xrandr_active_str.lines().collect::<Vec<&str>>()
        .drain(1..)
        .map(|row| {
            row.split_whitespace().last().expect("Doesn't last").to_owned()
        }).collect::<Vec<String>>()
}

fn set_brightness(display_name: &str, brightness: &f64) {
    Command::new("xrandr")
        .args(["--output", display_name, "--brightness", brightness.to_string().as_str()].iter())
        .output().unwrap();
}

fn main() {
    let available_display_names = get_display_names();

    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    let brightness_args = matches.values_of("brightness").expect("No brightnesses specified!");
    let brightnesses = brightness_args.map(|arg|
        arg_validation::float_in_range(arg, [0., 1.])
            .expect("Brightness should be a floating point value between 0 and 1"))
        .collect::<Vec<f64>>();

    let all = matches.is_present("all");

    let display_i_args = matches.values_of("display_index");

    let display_name_args = matches.values_of("display_name");

    let display_names = if all {
        available_display_names
    } else if display_name_args.is_some() {
        display_name_args.unwrap().map(|arg| arg.to_owned()).collect::<Vec<String>>()
    } else if display_i_args.is_some() {
        display_i_args.unwrap().map(|arg|
            arg_validation::limited_natural_num(arg, available_display_names.len())
                .expect("Display index should between 1 and the number of detected displays."))
            .map(|i| available_display_names[i - 1].to_owned()).collect::<Vec<String>>()
    } else {
        panic!("No display specified!");
    };

    if brightnesses.len() == 1 {
        let brightness = brightnesses.first().unwrap();
        display_names.iter().for_each(|display| set_brightness(display, brightness))
    } else if brightnesses.len() == display_names.len() {
        display_names.iter().zip(brightnesses.iter()).for_each(|(d, b)| set_brightness(d.as_str(), b))
    } else {
        let display_count = display_names.len();
        let brightness_count = brightnesses.len();
        panic!("Unable to determine brightnesses for displays. Either specify a single brightness value for all displays or one value per display\
        \nFound {} brightness values and {} displays", display_count, brightness_count);
    }

}
