# AI Code Dump (aidump) - Easy AI Coding

Turn codebase into a single file.

AI Code Dump (aidump) is a lightweight CLI tool that simplifies AI coding and
vibe coding.

It recursively collects files in a directory into a single text file that can be
easily added to the context of a prompt.

## Features

- Recursively walks entire project directory.
- Ignores files specified in `.gitignore` and `.dumpignore`.
- Includes a file paths before each file's content for context.
- Simple CLI interface.
- Formatted for easy interpretation by LLMs.

### Example Output

```
//--------------------------------------------------
// File: src/main.rs
//--------------------------------------------------
fn main() {
    println!("Hello, world!");
}

//--------------------------------------------------
// File: src/test.rs
//--------------------------------------------------
fn test() {
    assert_eq!(1, 1);
}
```

### Command-line Options

```
Collects a codebase into a single text file

Usage: aidump [OPTIONS] [PATH]

Arguments:
  [PATH]  Optional input directory path to process (defaults to current directory) [default: .]

Options:
  -o, --out <OUT>          Sets the output file name [default: code_dump.txt]
  -e, --exclude <EXCLUDE>  Glob patterns to exclude files (comma-separated)
  -v, --verbose            Show verbose logging output
  -u, --use-banner         Add a high-visibility banner to mark the top of each file. (default: true)
  -h, --help               Print help
  -V, --version            Print version
```

## Installation

### Using Cargo

If you have Rust and Cargo installed:

```bash
cargo install aidump
```

## Usage

```bash
# Process the current directory.
aidump

# Process a specific directory.
aidump /path/to/your/project

# Specify a custom output file.
aidump -o my_code_dump.txt

# Exclude files matching specific patterns
aidump -e "*.log,*.tmp,node_modules/*"
```

### Using `.dumpignore`

For convenience, AI Code Dump supports a `.dumpignore` file, which works
similarly to `.gitignore`.

Files matching glob patterns in `.dumpignore` will be excluded from the output.
Empty lines and lines starting with `#` are ignored.

```
# Ignore all folders named `secrets`.
secrets/

# Ignore the `tests` folder located in the same directory as the `.dumpignore` file.
/tests/

# Ignore all shell scripts.
*.sh
```

## Why AI Code Dump?

AI Code Dump was created to simplify the process of gathering code for
AI-assisted coding.

This makes it easy to collect the entire codebase so that it can be added to the
prompt's context.

Can be copied directly into the prompt, or added as an attachment to an LLM.

## License

This project is licensed under the MIT License - see the LICENSE file for
details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
