extern crate hex_d_hex;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::env::args;
use std::path::Path;
use std::error::Error;

enum PatcherError {
    OutputWriteError
}

fn search_replace(
    buf: &[u8],
    outfile: &mut File,
    matcher: &[u8],
    replacer: &[u8],
) -> Result<usize, PatcherError> {
    assert_eq!(matcher.len(), replacer.len());

    let mut i = 0;
    let mut substitutions = 0;

    while i < buf.len() {
        let matcher_end = i + matcher.len();
        if matcher_end < buf.len() && &buf[i..matcher_end] == matcher {
            match outfile.write(replacer) {
                Ok(_o) => {}
                Err(e) => {
                    println!("Could not write to output! - {}", e);
                    return Err(PatcherError::OutputWriteError);
                }
            }
            i += matcher.len();
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
    let argv: Vec<String> = args().collect();

    if args().len() != 5 {
        println!("Usage: {} file outfile old_pattern new_pattern", argv[0]);
        return;
    }

    let path = Path::new(&argv[1]);
    let outpath = Path::new(&argv[2]);
    let old_pattern = *hex_d_hex::dhex(&argv[3]);
    let new_pattern = *hex_d_hex::dhex(&argv[4]);

    if old_pattern.len() != new_pattern.len() {
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
        Err(e) => panic!("Could not create file! - {}", e)
    };

    match search_replace(&buf, &mut out, &old_pattern, &new_pattern) {
        Ok(o) => println!("Substituted {} occurences.", o),
        Err(_) => {}
    }
}
