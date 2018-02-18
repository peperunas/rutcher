extern crate clap;
extern crate hex_d_hex;

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use clap::{App, Arg};

fn search_replace(
    buf: &[u8],
    search: &[u8],
    replace: &[u8],
    mut output: File,
) -> Result<usize, std::io::Error> {
    assert_eq!(search.len(), replace.len());

    let mut i = 0;
    let mut substitutions = 0;

    while i < buf.len() {
        let search_end = i + search.len();
        if search_end < buf.len() && &buf[i..search_end] == search {
            output.write(replace)?;
            i += search.len();
            substitutions += 1;
        } else {
            output.write(&[buf[i]])?;
            i += 1;
        }
    }
    Ok(substitutions)
}

fn run(
    input_path: &Path,
    output_path: &Path,
    search: &[u8],
    replace: &[u8],
) -> Result<usize, std::io::Error> {
    let mut f = File::open(&input_path)?;
    let mut buf = Vec::new();
    let n: usize;

    println!("Opening \"{}\"", input_path.display());
    f.read_to_end(&mut buf)?;

    let out = File::create(output_path)?;
    n = search_replace(&buf, search, replace, out)?;

    println!("Substituted {} occurences.", n);
    Ok(n)
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
            Arg::with_name("search")
                .short("s")
                .required(true)
                .help("The pattern that has to be replaced")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("replace")
                .short("r")
                .required(true)
                .takes_value(true)
                .help("The new pattern to apply"),
        )
        .get_matches();

    let input = Path::new(matches.value_of("input").unwrap());
    let output = Path::new(matches.value_of("output").unwrap());
    let search = *hex_d_hex::dhex(&matches.value_of("search").unwrap());
    let replace = *hex_d_hex::dhex(&matches.value_of("replace").unwrap());

    if search.len() != replace.len() {
        println!("Patterns must have same length!");
        return;
    }
    run(input, output, &search, &replace).unwrap();
}
