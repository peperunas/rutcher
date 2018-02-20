extern crate hex_d_hex;
extern crate memmap;
extern crate regex;
#[macro_use]
extern crate structopt;

use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use regex::bytes::Regex;
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

fn build_regex_helper(pattern: &[u8]) -> String {
    let mut re = String::with_capacity(pattern.len() * 4 + 5);
    re.push_str("(?-u)");
    for b in pattern {
        re.push_str(&format!("\\x{:02x}", b));
    }
    re
}

fn build_regex(pattern: &[u8]) -> Regex {
    let re = build_regex_helper(pattern);

    // Shouldn't panic if there's no bug in `build_regex_helper`
    Regex::new(&re).expect("invalid regex")
}

fn search_replace<W: Write>(
    buf: &[u8],
    search: &[u8],
    replace: &[u8],
    mut output: W,
) -> Result<usize, std::io::Error> {
    let re = build_regex(search);
    let mut last = 0;
    let mut substitutions = 0;
    for cap in re.captures_iter(buf) {
        let m = cap.get(0).unwrap();
        output.write_all(&buf[last..m.start()])?;
        output.write_all(replace)?;
        last = m.end();

        substitutions += 1;
    }
    output.write_all(&buf[last..])?;

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

    // This will be undone later
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

#[test]
fn escape() {
    assert_eq!(build_regex_helper(b""), r#"(?-u)"#);
    assert_eq!(build_regex_helper(b"\xab\xcd\x01"), r#"(?-u)\xab\xcd\x01"#);
}

#[test]
fn replacement() {
    let test = |inp: &[u8], pat: &[u8], rep: &[u8], exp: &[u8], num| {
        let mut buf = Vec::new();
        assert_eq!(search_replace(inp, pat, rep, &mut buf).unwrap(), num);
        assert_eq!(buf, exp);
    };

    test(b"", b"CD", b"cd", b"", 0);
    test(b"aAa", b"A", b"d", b"ada", 1);
    test(b"aAa", b"", b"d", b"dadAdad", 4);
    test(b"CC", b"C", b"cc", b"cccc", 2);
    test(b"CC", b"C", b"CC", b"CCCC", 2);
    test(b"ABEF", b"CD", b"cd", b"ABEF", 0);
    test(b"ABCDEF", b"CD", b"cd", b"ABcdEF", 1);
    test(b"ABCDCDEF", b"CD", b"cd", b"ABcdcdEF", 2);
    test(b"ABCDECDF", b"CD", b"cd", b"ABcdEcdF", 2);
    test(b"ABCD", b"CD", b"cd", b"ABcd", 1);
}
