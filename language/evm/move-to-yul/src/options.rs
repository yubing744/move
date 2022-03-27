// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use codespan_reporting::diagnostic::Severity;

/// Options for a run of the compiler.
#[derive(Parser, Debug)]
#[clap(name = "move-to-yul", about = "Move Solidity Generator")]
pub struct Options {
    /// Directories where to lookup dependencies.
    #[clap(
        short,
        takes_value(true),
        multiple_values(true),
        multiple_occurrences(true)
    )]
    pub dependencies: Vec<String>,
    /// Named address mapping.
    #[clap(
        short,
        takes_value(true),
        multiple_values(true),
        multiple_occurrences(true)
    )]
    pub named_address_mapping: Vec<String>,
    /// Output file name.
    #[clap(short)]
    #[clap(long, default_value = "output.yul")]
    pub output: String,
    /// Solc executable
    #[clap(long, env = "SOLC_EXE", default_value = "solc")]

    pub solc_exe: String,
    /// Whether to dump bytecode to a file.
    #[clap(long = "dump-bytecode")]
    pub dump_bytecode: bool,
    /// Sources to compile (positional arg)
    pub sources: Vec<String>,
}

impl Default for Options {
    fn default() -> Self {
        Parser::parse_from(std::iter::empty::<String>())
    }
}

impl Options {
    pub fn report_severity(&self) -> Severity {
        Severity::Warning
    }

    pub fn version(&self) -> &str {
        "0.0"
    }
}
