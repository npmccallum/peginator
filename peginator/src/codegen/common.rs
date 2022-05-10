use anyhow::Result;
use quote::{format_ident, quote};

use proc_macro2::{Ident, Span, TokenStream};
use std::collections::BTreeSet;

pub struct CodegenSettings {
    pub skip_whitespace: bool,
    pub peginator_crate_name: String,
}

impl Default for CodegenSettings {
    fn default() -> Self {
        Self {
            skip_whitespace: true,
            peginator_crate_name: "peginator".into(),
        }
    }
}

pub trait CodegenGrammar {
    fn generate_code(&self, settings: &CodegenSettings) -> Result<TokenStream>;
}

pub trait CodegenRule {
    fn generate_code(&self, settings: &CodegenSettings)
        -> Result<(bool, TokenStream, TokenStream)>;
}

#[derive(Debug, Clone)]
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

pub trait Codegen {
    /// Generate code that's related to the parse function and the child parsers and types.
    ///
    /// Should not generate any types that are related to this parser.
    fn generate_code_spec(
        &self,
        rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<TokenStream>;

    /// Get all fields that are generated by this parser
    fn get_fields(&self) -> Result<Vec<FieldDescriptor>>;

    /// Generate all parse code and types that are related to this parser
    ///
    /// Calls generate_code_spec and also generates related types
    fn generate_code(
        &self,
        rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let spec_body = self.generate_code_spec(rule_fields, settings)?;
        let parsed_type = self.generate_struct_type(rule_fields, settings, "Parsed")?;
        Ok(quote!(
            #spec_body
            #parsed_type
        ))
    }

    fn generate_struct_type(
        &self,
        rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
        type_name: &str,
    ) -> Result<TokenStream> {
        let fields = self.get_filtered_rule_fields(rule_fields)?;
        Ok(generate_parsed_struct_type(type_name, &fields, settings))
    }

    fn get_filtered_rule_fields<'a>(
        &self,
        rule_fields: &[FieldDescriptor<'a>],
    ) -> Result<Vec<FieldDescriptor<'a>>> {
        let fields = self.get_fields()?;
        Ok(rule_fields
            .iter()
            .filter(|rf| fields.iter().any(|f| f.name == rf.name))
            .cloned()
            .collect())
    }
}

fn generate_parsed_struct_type(
    type_name: &str,
    fields: &[FieldDescriptor],
    settings: &CodegenSettings,
) -> TokenStream {
    let type_ident = format_ident!("{}", type_name);
    if fields.is_empty() {
        quote!(
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct #type_ident;
        )
    } else {
        let field_names: Vec<Ident> = fields
            .iter()
            .map(|f| Ident::new(f.name, Span::call_site()))
            .collect();
        let field_types: Vec<TokenStream> = fields
            .iter()
            .map(|f| generate_field_type(type_name, f, settings))
            .collect();
        quote!(
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct #type_ident {
                #( pub #field_names: #field_types, )*
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
        let ident = format_ident!("{}_{}", parent_type, field.name);
        quote!(#ident)
    } else {
        let type_name = field.type_names.iter().next().unwrap();
        let ident = format_ident!("{}", type_name);
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
    parent_type: &str,
    field: &FieldDescriptor,
    _settings: &CodegenSettings,
) -> TokenStream {
    let ident = format_ident!("{}_{}", parent_type, field.name);
    let type_idents: Vec<Ident> = field
        .type_names
        .iter()
        .map(|n| format_ident!("{}", n))
        .collect();
    quote!(
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum #ident {
            #(#type_idents(#type_idents),)*
        }
    )
}
