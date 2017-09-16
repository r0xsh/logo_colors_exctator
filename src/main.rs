extern crate image;
extern crate clap;

use std::path::Path;
use image::Rgb;

use clap::{App, Arg};


const CHARS: &'static [u8] = b"0123456789abcdef";
const CHARS_UPPERCASE: &'static [u8] = b"0123456789ABCDEF";

fn parse_args() -> clap::ArgMatches<'static> {
    App::new("Logo color extract")
        .version("1.0")
        .about("Exctact all colors in hexadecimal from a logo")
        .author("Antoine Bagnaud <bagnaud.antoine@gmail.com>")
        .arg(Arg::with_name("FILE")
             .required(true)
             .takes_value(true)
             .index(1)
             .help("Path to a image file"))
        .arg(Arg::with_name("uppercase")
             .short("u")
             .long("uppercase")
             .help("Return hexadecimal with uppercase letters #4ADE65"))
        .get_matches()
}

fn in_vector(vec: &Vec<String>, find: &String) -> bool {
    for vec in vec.into_iter() {
        if vec == find {
            return true
        }
    }
    false
}

fn pixel_2_hex(pix: &Rgb<u8>, uppercase: bool) -> String {
    let mut v = Vec::with_capacity(6);
    for &byte in pix.data.into_iter() {
        if !uppercase {
            v.push(CHARS[(byte >> 4) as usize]);
            v.push(CHARS[(byte & 0xf) as usize]);
        } else {
            v.push(CHARS_UPPERCASE[(byte >> 4) as usize]);
            v.push(CHARS_UPPERCASE[(byte & 0xf) as usize]);
        }
    }

    unsafe {
        String::from_utf8_unchecked(v)
    }
}

fn main() {
    let matches = parse_args();

    let path = matches.value_of("FILE").unwrap();

    let image = image::open(&Path::new(path)).unwrap();
    let rgb = image.to_rgb();
    let pixels = rgb.pixels();

    let mut values: Vec<String> = Vec::new();
    for p in pixels {
        let hex = pixel_2_hex(&p, matches.is_present("uppercase"));
        if !in_vector(&values, &hex) {
            values.push(hex);
        }
    }

    for hex in values {
        println!("#{}", hex);
    }
}
