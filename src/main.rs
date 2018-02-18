extern crate clap;
extern crate hex_d_hex;

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use clap::{App, Arg};

fn search_replace(
    buf: &[u8],
    outfile: &mut File,
    search_pattern: &[u8],
    replace_pattern: &[u8],
) -> Result<usize, std::io::Error> {
    assert_eq!(search_pattern.len(), replace_pattern.len());

    let mut i = 0;
    let mut substitutions = 0;

    while i < buf.len() {
        let search_pattern_end = i + search_pattern.len();
        if search_pattern_end < buf.len() && &buf[i..search_pattern_end] == search_pattern {
            outfile.write(replace_pattern)?;
            i += search_pattern.len();
            substitutions += 1;
        } else {
            outfile.write(&[buf[i]])?;
            i += 1;
        }
    }
    Ok(substitutions)
}

fn run(input: &Path, output: &Path, search: &[u8], replace: &[u8]) -> Result<(), std::io::Error> {
    let mut f = File::open(&input)?;
    let mut buf: Vec<u8> = Vec::new();

    println!("Opening \"{}\"", input.display());

    f.read_to_end(&mut buf)?;

    let mut out = File::create(output)?;

    match search_replace(&buf, &mut out, &search, &replace) {
        Ok(o) => {
            println!("Substituted {} occurences.", o);
            Ok(())
        },
        Err(e) => Err(e)
    }
}

fn main() {
    let matches = App::new("rutcher")
        .version("0.1.2")
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

    let input = Path::new(matches.value_of("input").unwrap());
    let output = Path::new(matches.value_of("output").unwrap());
    let search = *hex_d_hex::dhex(&matches.value_of("search_pattern").unwrap());
    let replace = *hex_d_hex::dhex(&matches.value_of("replace_pattern").unwrap());

    if search.len() != replace.len() {
        println!("Patterns must have same length!");
        return;
    }
    run(input, output, &search, &replace).unwrap();
}
