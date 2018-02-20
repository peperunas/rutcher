extern crate hex_d_hex;
extern crate memmap;
#[macro_use]
extern crate structopt;

use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use memmap::Mmap;

/// A simple program which searches for a pattern in a file and patches it with a new one
#[derive(StructOpt, Debug)]
struct Options {
    /// Input file
    #[structopt(parse(from_os_str))]
    input_file: PathBuf,
    /// Output file
    #[structopt(parse(from_os_str))]
    output_file: PathBuf,
    /// Pattern to search for
    #[structopt(short = "s", long = "pattern")]
    pattern: String,
    /// Replacement pattern
    #[structopt(short = "r", long = "replacement")]
    replacement: String,
}

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
    println!(r#"Opening "{}""#, input_path.display());

    let f = File::open(&input_path)?;
    let buf = unsafe { Mmap::map(&f)? };

    let out = File::create(output_path)?;
    let n = search_replace(&buf, search, replace, out)?;

    println!("Substituted {} occurences.", n);
    Ok(n)
}

fn main() {
    let options = Options::from_args();

    let pattern = hex_d_hex::dhex(&options.pattern);
    let replacement = hex_d_hex::dhex(&options.replacement);
    if pattern.len() != replacement.len() {
        println!("Patterns must have same length!");
        return;
    }

    run(
        &options.input_file,
        &options.output_file,
        &pattern,
        &replacement,
    ).expect("IO error");
}
