// Copyright (c) 2025 CodeDump Contributors
// Licensed under the MIT License

//! AI CodeDump - Collects a codebase into a single text file.
//!
//! This tool converts a codebase into a single text file for AI coding use.

mod error;
use clap::Parser;
use error::CodeDumpError;
use glob::Pattern;
use ignore::WalkBuilder;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Write};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[clap(author, version, about = "Collects a codebase into a single text file")]
struct Args {
    /// Optional input directory path to process (defaults to current directory).
    #[clap(value_parser, default_value = ".")]
    path: PathBuf,

    /// Sets the output file name.
    #[clap(short = 'o', long, default_value = "code_dump.txt")]
    out: String,

    /// Glob patterns to exclude files (comma-separated).
    #[clap(short = 'e', long)]
    exclude: Option<String>,

    /// Show verbose logging output.
    #[clap(short, long)]
    verbose: bool,

    /// Add a high-visibility banner to mark the top of each file. (default: true)
    #[clap(short, long, default_value = "true")]
    use_banner: bool,
}

fn parse_exclude_patterns(patterns_str: Option<String>) -> Result<Vec<Pattern>, CodeDumpError> {
    let mut patterns = Vec::new();
    if let Some(patterns_str) = patterns_str {
        for pattern in patterns_str.split(',') {
            let pattern = pattern.trim();
            match Pattern::new(pattern) {
                Ok(p) => patterns.push(p),
                Err(_) => return Err(CodeDumpError::Pattern(pattern.to_string())),
            }
        }
    }
    Ok(patterns)
}

fn filter_files(
    input_path: &Path,
    additional_excludes: &[Pattern],
) -> impl Iterator<Item = PathBuf> {
    let walker = WalkBuilder::new(input_path)
        .add_custom_ignore_filename(".dumpignore")
        .build();

    walker
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .map(|entry| entry.into_path())
        .filter(|path| {
            !additional_excludes.iter().any(|pattern| {
                let p = path.strip_prefix("./").unwrap_or(path);
                pattern.matches_path(&p)
            })
        })
}

fn dump_files<I: IntoIterator<Item = PathBuf>, P: AsRef<Path>>(
    files: I,
    output_path: &P,
    verbose: bool,
    use_large_banner: bool,
) -> io::Result<usize> {
    let mut file_count = 0;

    if let Some(parent) = &output_path.as_ref().parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }
    let mut outfile = File::create(&output_path)?;

    for f in files {
        let section_head = if use_large_banner {
            banner(&f.display().to_string())
        } else {
            format!("\n// File: {}\n", f.display())
        };

        match fs::read_to_string(&f) {
            Ok(content) => {
                writeln!(outfile, "{}{}", section_head, content)?;
                if verbose {
                    println!("Add: {}", f.display());
                }
                file_count += 1;
            }
            Err(e) => {
                if e.kind() == ErrorKind::InvalidData {
                    if verbose {
                        println!("Skipping binary file: {}", f.display());
                    }
                } else {
                    eprintln!("Error reading {}: {}", f.display(), e);
                }
            }
        }
    }

    Ok(file_count)
}

fn banner(text: &str) -> String {
    format!(
        "//---------------------------------------------------\n\
         // File: {}\n\
         //---------------------------------------------------\n",
        text
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut exclude_patterns = match parse_exclude_patterns(args.exclude.clone()) {
        Ok(patterns) => patterns,
        Err(e) => return Err(Box::new(e)),
    };

    // Exclude the output file.
    exclude_patterns.push(Pattern::new(&args.out).unwrap());

    let files = filter_files(&args.path, &exclude_patterns);

    let file_count = dump_files(files, &args.out, args.verbose, args.use_banner)?;

    println!("Collected {} files into {}", file_count, &args.out);

    Ok(())
}
