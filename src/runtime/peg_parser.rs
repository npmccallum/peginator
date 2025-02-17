// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use super::{IndentedTracer, NoopTracer, ParseError, ParseTracer};

/// The main trait for interfacing with peginator. Implemented by `@export`-ed rules.
pub trait PegParser: Sized {
    /// Parse a string into the AST.
    fn parse(s: &str) -> Result<Self, ParseError>;

    /// Parse a string into the AST, print a colored trace of the parse process.
    ///
    /// The printing happens with regular `eprintln!()`.
    fn parse_with_trace(s: &str) -> Result<Self, ParseError>;
}

impl<T: PegParserAdvanced<()>> PegParser for T {
    fn parse(s: &str) -> Result<Self, ParseError> {
        Self::parse_advanced::<NoopTracer>(s, &ParseSettings::default(), ())
    }
    fn parse_with_trace(s: &str) -> Result<Self, ParseError> {
        Self::parse_advanced::<IndentedTracer>(s, &ParseSettings::default(), ())
    }
}

// Internal trait for generated code
pub trait PegParserAdvanced<TUD>: Sized {
    /// Internal function that is actually generated by the grammar compiler, used by the more
    /// friendly functions.
    fn parse_advanced<TT: ParseTracer>(
        s: &str,
        settings: &ParseSettings,
        user_defined: TUD,
    ) -> Result<Self, ParseError>;
}

/// Parse settings (for future compatibility)
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ParseSettings {}
