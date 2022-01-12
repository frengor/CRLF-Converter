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
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use itertools::Itertools;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "CRLF-Converter", about, author)]
struct Args {
    /// The file(s) to convert
    #[structopt(name = "file-to-convert", required(true), parse(from_os_str))]
    paths: Vec<PathBuf>,
    /// Every CRLF in the file(s) will be converted to LF. This is the default option
    #[structopt(name = "crlf-to-lf", long)]
    crlf_to_lf: bool,
    /// Every LF in the file(s) will be converted to CRLF
    #[structopt(name = "lf-to-crlf", long)]
    lf_to_crlf: bool,
}

fn main() -> Result<()> {
    let mut args: Args = Args::from_args();
    {
        let current_dir = std::env::current_dir().with_context(|| format!("Cannot get current directory"))?;
        args.paths = args.paths.into_iter().map(|path| {
            if !path.is_absolute() {
                let mut current_dir = current_dir.clone();
                current_dir.push(path);
                current_dir
            } else {
                path
            }
        }).filter(|path| {
            if !path.exists() {
                eprintln!(r#"File "{}" does not exists"#, path.display());
                return false;
            }

            if !path.is_file() {
                eprintln!(r#"File "{}" is not a valid file to convert"#, path.display());
                return false;
            }

            true
        }).collect();
    }

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
    string.chars()
    .peekable()
    .batching(|c| {
        match c.next() {
            Some('\r') => {
                match c.peek() {
                    Some('\n') => {
                        // Actually consume '\n'
                        c.next();
                        Some('\n')
                    },
                    _ => Some('\r'),
                }
            }
            x => x,
        }
    })
    .collect()
}

fn lf_to_crlf(string: &str) -> String {
    let mut out_n = false;
    string.chars()
    .peekable()
    .batching(|c| {
        if out_n {
            out_n = false;
            Some('\n')
        } else {
            match c.next() {
                Some('\r') => {
                    if let Some('\n') = c.peek() {
                        // Actually consume '\n'
                        c.next();
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
    })
    .collect()
}
