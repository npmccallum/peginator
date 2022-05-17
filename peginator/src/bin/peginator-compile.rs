// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use colored::*;
use std::fs;

use anyhow::Result;
use clap::Parser;

use peginator::codegen::{generate_source_header, CodegenGrammar, CodegenSettings, Grammar};
use peginator::{PegParser, PrettyParseError};

/// Compile EBNF grammar into rust parser code.
#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    /// Module path of the built-in peginator code
    #[clap(short, long, default_value_t = String::from("peginator"))]
    peginator_crate_name: String,

    /// Print the parsed AST and exit
    #[clap(short, long)]
    ast_only: bool,

    /// Trace rule matching
    #[clap(short, long)]
    trace: bool,

    grammar_file: String,
}

fn main_wrap() -> Result<()> {
    let args = Args::parse();
    let grammar = fs::read_to_string(&args.grammar_file)?;
    let parsed_grammar = if args.trace {
        Grammar::parse_with_trace(&grammar)
    } else {
        Grammar::parse(&grammar)
    }
    .map_err(|err| PrettyParseError::from_parse_error(&err, &grammar, Some(&args.grammar_file)))?;
    if args.ast_only {
        println!("{:#?}", parsed_grammar);
        return Ok(());
    }

    let settings = CodegenSettings {
        skip_whitespace: true,
        peginator_crate_name: args.peginator_crate_name,
    };
    let generated_code = parsed_grammar.generate_code(&settings)?;
    println!("{}", generate_source_header(&grammar, false));
    println!("{}", generated_code);
    Ok(())
}

fn main() {
    if let Err(e) = main_wrap() {
        println!("{}: {}", "Error".red().bold(), e)
    }
}
