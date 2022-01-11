use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use itertools::Itertools;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "crlfconverter", about, author)]
struct Args {
    /// The file to convert
    #[structopt(name = "file-to-convert", parse(from_os_str))]
    path: PathBuf,
    /// Every CRLF in the file will be converted to LF
    #[structopt(name = "crlf-to-lf", long)]
    crlf_to_lf: bool,
    /// Every LF in the file will be converted to CRLF
    #[structopt(name = "lf-to-crlf", long)]
    lf_to_crlf: bool,
}

fn main() -> Result<()> {
    let mut args: Args = Args::from_args();
    {
        let path = &args.path;
        if !path.is_absolute() {
            let mut current_dir = std::env::current_dir().with_context(|| format!("Cannot get current directory"))?;
            current_dir.push(path);
            args.path = current_dir;
        }

        if !args.path.exists() {
            bail!(r#"File "{}" does not exists"#, args.path.display());
        }

        if !args.path.is_file() {
            bail!(r#"File "{}" is not a valid file to convert"#, args.path.display());
        }
    }
    match args {
        Args { crlf_to_lf: true, lf_to_crlf: true, .. } => bail!("--crlf-to-lf and --lf-to-crlf cannot be enabled at the same time"),
        Args { path, lf_to_crlf: true, .. } => modify_content(&path, lf_to_crlf),
        Args { path, .. } => modify_content(&path, crlf_to_lf),
    }
}

fn modify_content(path: &Path, f: impl FnOnce(&str) -> String) -> Result<()> {
    let str = fs::read_to_string(&path).with_context(|| format!(r#"Failed to read from "{}""#, path.display()))?;
    let str = f(&str);
    fs::write(&path, str).with_context(|| format!(r#"Failed to write to "{}""#, path.display()))
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
