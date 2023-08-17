//   Copyright 2022 fren_gor <goro@frengor.com>
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.

use std::fs;
use std::iter::{Extend, from_fn};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "CRLF-Converter", author, version, about, long_about = None)]
struct Args {
    /// The file(s) to convert
    #[arg(value_name = "file-to-convert")]
    paths: Vec<PathBuf>,
    /// Every CRLF in the file(s) will be converted to LF. This is the default option
    #[arg(long)]
    crlf_to_lf: bool,
    /// Every LF in the file(s) will be converted to CRLF
    #[arg(long)]
    lf_to_crlf: bool,
}

fn main() -> Result<()> {
    let mut args: Args = Args::parse();
    args.paths = args.paths.into_iter()
    .filter(|path| {
        if !path.exists() {
            eprintln!(r#"File "{}" does not exists"#, path.display());
            return false;
        }

        if !path.is_file() {
            eprintln!(r#"File "{}" is not a valid file to convert"#, path.display());
            return false;
        }

        true
    })
    .collect();

    if args.paths.is_empty() {
        bail!("No valid files have been provided.");
    }

    match args {
        Args { crlf_to_lf: true, lf_to_crlf: true, .. } => bail!("--crlf-to-lf and --lf-to-crlf cannot be enabled at the same time"),
        Args { paths, lf_to_crlf: true, .. } => convert(&paths, lf_to_crlf),
        Args { paths, .. } => convert(&paths, crlf_to_lf),
    }
}

fn convert(paths: &Vec<PathBuf>, f: impl Fn(&str) -> String) -> Result<()> {
    for path in paths {
        modify_content(path, &f)?;
        println!(r#""{}" has been converted."#, path.display());
    }
    Ok(())
}

fn modify_content(path: &Path, f: impl Fn(&str) -> String) -> Result<()> {
    let str = fs::read_to_string(&path).with_context(|| format!(r#"Failed to read from "{}""#, path.display()))?;
    let str = f(&str);
    fs::write(&path, &str).with_context(|| format!(r#"Failed to write to "{}""#, path.display()))
}

fn crlf_to_lf(string: &str) -> String {
    let mut it = string.chars().peekable();
    let iter = from_fn(|| {
        match it.next() {
            Some('\r') => {
                match it.peek() {
                    Some('\n') => {
                        // Actually consume '\n'
                        it.next();
                        Some('\n')
                    },
                    _ => Some('\r'),
                }
            }
            x => x,
        }
    });
    let mut str = String::with_capacity(string.len());
    str.extend(iter);
    str
}

fn lf_to_crlf(string: &str) -> String {
    let mut out_n = false;
    let mut prev_r = false;
    // Calculate the capacity for the returned String
    // Preferring spending some time here instead of allocating more heap since files can be very large
    let final_capacity = string.len() + string.chars().filter(|&c| {
        match (prev_r, c) {
            (_, '\r') => { prev_r = true; false },
            (true, _) => { prev_r = false; false },
            (_, '\n') => true,
            _ => false,
        }
    }).count();

    let mut it = string.chars().peekable();
    let iter = from_fn(|| {
        if out_n {
            out_n = false;
            Some('\n')
        } else {
            match it.next() {
                Some('\r') => {
                    if let Some('\n') = it.peek() {
                        // Actually consume '\n'
                        it.next();
                        out_n = true;
                    }
                    Some('\r')
                },
                Some('\n') => {
                    out_n = true;
                    Some('\r')
                },
                x => x,
            }
        }
    });
    // Don't allocate more than needed
    let mut str = String::with_capacity(final_capacity);
    str.extend(iter);
    str
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn crlf_to_lf_test() {
        assert_eq!(crlf_to_lf("Hello!\r\nHow\r\nare\ryou\n?\n\r"), String::from("Hello!\nHow\nare\ryou\n?\n\r"));
    }

    #[test]
    fn lf_to_crlf_test() {
        assert_eq!(lf_to_crlf("Hello!\n\nHow\r\nare\ryou\n?\n\r"), String::from("Hello!\r\n\r\nHow\r\nare\ryou\r\n?\r\n\r"));
    }
}
