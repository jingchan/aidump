// Copyright (c) 2025 CodeDump Contributors
// Licensed under the MIT License

//! AI CodeDump - Collects a codebase into a single text file.
//!
//! This tool converts a codebase into a single text file for AI coding use.

#[derive(Debug)]
pub enum CodeDumpError {
    Io(std::io::Error),
    Pattern(String),
}

impl From<std::io::Error> for CodeDumpError {
    fn from(err: std::io::Error) -> Self {
        CodeDumpError::Io(err)
    }
}

impl std::fmt::Display for CodeDumpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeDumpError::Io(err) => write!(f, "I/O error: {}", err),
            CodeDumpError::Pattern(pattern) => write!(f, "Invalid glob pattern: {}", pattern),
        }
    }
}

impl std::error::Error for CodeDumpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CodeDumpError::Io(err) => Some(err),
            _ => None,
        }
    }
}
