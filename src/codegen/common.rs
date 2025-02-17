// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use std::{any::type_name, collections::BTreeSet};

use anyhow::Result;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};

use crate::grammar::Grammar;

#[derive(Debug, Clone)]
pub struct CodegenSettings {
    pub skip_whitespace: bool,
    pub peginator_crate_name: String,
    pub derives: Vec<String>,
    pub user_defined_type: TokenStream,
}

impl Default for CodegenSettings {
    fn default() -> Self {
        Self {
            skip_whitespace: true,
            peginator_crate_name: "peginator".into(),
            derives: vec!["Debug".into(), "Clone".into()],
            user_defined_type: quote!(()),
        }
    }
}

impl CodegenSettings {
    pub fn set_user_defined_type(&mut self, t: &str) {
        let idents = t.split("::").map(safe_ident);
        self.user_defined_type = quote!(#(#idents)::*);
    }
}

pub trait CodegenGrammar {
    fn generate_code(&self, settings: &CodegenSettings) -> Result<TokenStream>;
}

pub trait CodegenRule {
    fn generate_code(
        &self,
        grammar: &Grammar,
        settings: &CodegenSettings,
    ) -> Result<(TokenStream, TokenStream)>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Arity {
    One,
    Optional,
    Multiple,
}

#[derive(Debug, Clone)]
pub struct FieldDescriptor<'a> {
    pub name: &'a str,
    pub type_names: BTreeSet<&'a str>,
    pub arity: Arity,
    pub boxed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordPosition {
    No,
    Yes,
}

impl From<bool> for RecordPosition {
    fn from(b: bool) -> Self {
        if b {
            Self::Yes
        } else {
            Self::No
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicType {
    No,
    Yes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloneState {
    No,
    Yes,
}

pub trait Codegen {
    /// Generate code that's related to the parse function and the child parsers and types.
    ///
    /// Should not generate any types that are related to this parser.
    fn generate_code_spec(
        &self,
        rule_fields: &[FieldDescriptor],
        grammar: &Grammar,
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let _ = grammar;
        if let Some(parse_body) =
            self.generate_inline_body(rule_fields, grammar, settings, CloneState::No)?
        {
            Ok(generate_inner_parse_function(parse_body, settings))
        } else {
            panic!(
                "Neither generate_code_spec, nor generate_inline_body was implemented for {}",
                type_name::<Self>(),
            );
        }
    }

    /// Get all fields that are generated by this parser
    fn get_fields<'a>(&'a self, grammar: &'a Grammar) -> Result<Vec<FieldDescriptor<'a>>>;

    /// Generate all parse code and types that are related to this parser
    ///
    /// Calls generate_code_spec and also generates related types
    fn generate_code(
        &self,
        rule_fields: &[FieldDescriptor],
        grammar: &Grammar,
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let spec_body = self.generate_code_spec(rule_fields, grammar, settings)?;
        let parsed_type = self.generate_struct_type(
            rule_fields,
            grammar,
            settings,
            "Parsed",
            RecordPosition::No,
            PublicType::No,
        )?;
        Ok(quote!(
            #spec_body
            #parsed_type
        ))
    }

    /// Generate an inline call without generating the whole body. Returns None if this is not possible
    fn generate_inline_body(
        &self,
        rule_fields: &[FieldDescriptor],
        grammar: &Grammar,
        settings: &CodegenSettings,
        clone_state: CloneState,
    ) -> Result<Option<TokenStream>> {
        let _ = (rule_fields, grammar, settings, clone_state);
        Ok(None)
    }

    fn generate_struct_type(
        &self,
        rule_fields: &[FieldDescriptor],
        grammar: &Grammar,
        settings: &CodegenSettings,
        type_name: &str,
        record_position: RecordPosition,
        public_type: PublicType,
    ) -> Result<TokenStream> {
        let fields = self.get_filtered_rule_fields(rule_fields, grammar)?;
        Ok(generate_parsed_struct_type(
            type_name,
            &fields,
            settings,
            record_position,
            public_type,
        ))
    }

    fn get_filtered_rule_fields<'a>(
        &self,
        rule_fields: &[FieldDescriptor<'a>],
        grammar: &Grammar,
    ) -> Result<Vec<FieldDescriptor<'a>>> {
        let fields = self.get_fields(grammar)?;
        Ok(rule_fields
            .iter()
            .filter(|rf| fields.iter().any(|f| f.name == rf.name))
            .cloned()
            .collect())
    }
}

pub fn generate_skip_ws(
    settings: &CodegenSettings,
    parse_fn_name: &str,
    additional_params: TokenStream,
    clone_state: CloneState,
) -> TokenStream {
    let parse_fn_ident = safe_ident(parse_fn_name);
    let state = match clone_state {
        CloneState::No => quote!(state),
        CloneState::Yes => quote!(state.clone()),
    };
    if settings.skip_whitespace {
        quote!(
            parse_Whitespace( #state, &mut * global ).and_then(|ParseOk{state, ..}| {
                #parse_fn_ident (state, #additional_params)
            })
        )
    } else {
        quote!( #parse_fn_ident (#state, #additional_params))
    }
}
pub fn generate_derives(settings: &CodegenSettings) -> TokenStream {
    if settings.derives.is_empty() {
        return quote!();
    }
    let derive_idents: Vec<Ident> = settings
        .derives
        .iter()
        .map(|f| Ident::new(f, Span::call_site()))
        .collect();
    quote!(#[derive( #( #derive_idents, )*)])
}

fn generate_parsed_struct_type(
    type_name: &str,
    fields: &[FieldDescriptor],
    settings: &CodegenSettings,
    record_position: RecordPosition,
    public_type: PublicType,
) -> TokenStream {
    let type_ident = safe_ident(type_name);
    let derives = if public_type == PublicType::Yes {
        generate_derives(settings)
    } else {
        quote!()
    };

    if fields.is_empty() && record_position == RecordPosition::No {
        match public_type {
            PublicType::No => quote!(pub type #type_ident = ();),
            PublicType::Yes => quote!(
                #derives
                pub struct #type_ident;
            ),
        }
    } else if fields.len() == 1
        && record_position == RecordPosition::No
        && public_type == PublicType::No
    {
        let field_type = generate_field_type(type_name, &fields[0], settings);
        quote!(pub type #type_ident = #field_type;)
    } else {
        let field_names: Vec<Ident> = fields.iter().map(|f| safe_ident(f.name)).collect();
        let field_types: Vec<TokenStream> = fields
            .iter()
            .map(|f| generate_field_type(type_name, f, settings))
            .collect();
        let position_field = if record_position == RecordPosition::Yes {
            quote!(pub position: std::ops::Range<usize>,)
        } else {
            quote!()
        };
        quote!(
            #derives
            pub struct #type_ident {
                #( pub #field_names: #field_types, )*
                #position_field
            }
        )
    }
}

pub fn generate_field_type(
    parent_type: &str,
    field: &FieldDescriptor,
    _settings: &CodegenSettings,
) -> TokenStream {
    let field_inner_type_ident: TokenStream = if field.type_names.len() > 1 {
        let field_name = &field.name;
        let ident = format_ident!("{parent_type}_{field_name}");
        quote!(#ident)
    } else {
        let type_name = field.type_names.iter().next().unwrap();
        let ident = safe_ident(type_name);
        if type_name == &"char" {
            quote!(char)
        } else {
            quote!(#ident)
        }
    };
    match field.arity {
        Arity::One => {
            if field.boxed {
                quote!(Box<#field_inner_type_ident>)
            } else {
                quote!(#field_inner_type_ident)
            }
        }
        Arity::Optional => {
            if field.boxed {
                quote!(Option<Box<#field_inner_type_ident>>)
            } else {
                quote!(Option<#field_inner_type_ident>)
            }
        }
        Arity::Multiple => quote!(Vec<#field_inner_type_ident>),
    }
}

pub fn generate_enum_type(
    name: &str,
    field: &FieldDescriptor,
    settings: &CodegenSettings,
) -> TokenStream {
    let ident = safe_ident(name);
    let derives = generate_derives(settings);
    let type_idents: Vec<Ident> = field.type_names.iter().map(safe_ident).collect();
    quote!(
        #[allow(non_camel_case_types)]
        #derives
        pub enum #ident {
            #(#type_idents(#type_idents),)*
        }
    )
}

pub fn generate_inner_parse_function(
    parse_body: TokenStream,
    settings: &CodegenSettings,
) -> TokenStream {
    let user_defined_type = &settings.user_defined_type;
    quote!(
        #[inline(always)]
        pub fn parse<'a, TT: ParseTracer>(
            state: ParseState<'a>,
            global: &mut ParseGlobal<TT, ParseCache<'a>, #user_defined_type>,
        ) -> ParseResult<'a, Parsed> {
            #parse_body
        }
    )
}

pub fn generate_rule_parse_function(
    parser_name: Ident,
    rule_type: Ident,
    parse_body: TokenStream,
    settings: &CodegenSettings,
) -> TokenStream {
    let user_defined_type = &settings.user_defined_type;
    quote!(
        #[inline]
        pub(super) fn #parser_name <'a, TT: ParseTracer>(
            state: ParseState<'a>,
            global: &mut ParseGlobal<TT, ParseCache<'a>, #user_defined_type>,
        ) -> ParseResult<'a, #rule_type> {
            #parse_body
        }
    )
}

pub fn safe_ident(name: impl AsRef<str>) -> Ident {
    let name = name.as_ref();
    if RUST_KEYWORDS.contains(&name) {
        format_ident!("r#{name}")
    } else {
        format_ident!("{name}")
    }
}

/// https://doc.rust-lang.org/reference/keywords.html
pub const RUST_KEYWORDS: [&str; 50] = [
    // "crate" can't be r#crate
    "as", "break", "const", "continue", "else", "enum", "extern", "false", "fn", "for", "if",
    "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self",
    "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
];
