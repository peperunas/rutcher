extern crate clap;
extern crate hex_d_hex;

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::error::Error;
use clap::{App, Arg};

enum PatcherError {
    OutputWriteError,
}

fn search_replace(
    buf: &[u8],
    outfile: &mut File,
    search_pattern: &[u8],
    replace_pattern: &[u8],
) -> Result<usize, PatcherError> {
    assert_eq!(search_pattern.len(), replace_pattern.len());

    let mut i = 0;
    let mut substitutions = 0;

    while i < buf.len() {
        let search_pattern_end = i + search_pattern.len();
        if search_pattern_end < buf.len() && &buf[i..search_pattern_end] == search_pattern {
            match outfile.write(replace_pattern) {
                Ok(_o) => {}
                Err(e) => {
                    println!("Could not write to output! - {}", e);
                    return Err(PatcherError::OutputWriteError);
                }
            }
            i += search_pattern.len();
            substitutions += 1;
        } else {
            match outfile.write(&[buf[i]]) {
                Ok(_) => {}
                Err(e) => {
                    println!("Could not write to output! - {}", e);
                    return Err(PatcherError::OutputWriteError);
                }
            }
            i += 1;
        }
    }
    Ok(substitutions)
}

fn main() {
    let matches = App::new("Rutcher")
        .version("0.1.1")
        .author("Giulio \"peperunas\" De Pasquale, <me@giugl.io>")
        .about(
            "A simple program which searches for a pattern in a file and patches it with a new one",
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .help("The file to patch")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .help("The file saved as output")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("search_pattern")
                .short("s")
                .required(true)
                .help("The pattern that has to be replaced")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("replace_pattern")
                .short("r")
                .required(true)
                .takes_value(true)
                .help("The new pattern to apply"),
        )
        .get_matches();

    let path = Path::new(matches.value_of("input").unwrap());
    let outpath = Path::new(matches.value_of("output").unwrap());
    let search_pattern = *hex_d_hex::dhex(&matches.value_of("search_pattern").unwrap());
    let replace_pattern = *hex_d_hex::dhex(&matches.value_of("replace_pattern").unwrap());

    if search_pattern.len() != replace_pattern.len() {
        println!("Patterns must have same length!");
        return;
    }

    println!("Opening \"{}\"", path.display());
    let mut f = match File::open(&path) {
        Err(why) => panic!(
            "Failed to open {}: {}",
            path.display(),
            Error::description(&why)
        ),
        Ok(f) => f,
    };

    let mut buf: Vec<u8> = Vec::new();

    match f.read_to_end(&mut buf) {
        Ok(_) => {}
        Err(_e) => {
            panic!("Could not read from file");
        }
    }

    let mut out = match File::create(outpath) {
        Ok(o) => o,
        Err(e) => panic!("Could not create file! - {}", e),
    };

    match search_replace(&buf, &mut out, &search_pattern, &replace_pattern) {
        Ok(o) => println!("Substituted {} occurences.", o),
        Err(_) => {}
    }
}
