// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use anyhow::Result;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

use super::common::{
    generate_field_type, safe_ident, Arity, Codegen, CodegenSettings, FieldDescriptor,
};
use crate::grammar::{Closure, Grammar};

impl Codegen for Closure {
    fn generate_code_spec(
        &self,
        rule_fields: &[FieldDescriptor],
        grammar: &Grammar,
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let closure_body = self.body.generate_code(rule_fields, grammar, settings)?;
        let fields = self.body.get_filtered_rule_fields(rule_fields, grammar)?;
        let declarations: TokenStream = fields
            .iter()
            .map(|f| {
                let typ = generate_field_type("Parsed", f, settings);
                let name_ident = safe_ident(f.name);
                quote!(let mut #name_ident: #typ = Vec::new();)
            })
            .collect();
        let assignments: TokenStream = fields
            .iter()
            .map(|field| {
                let name = safe_ident(field.name);
                quote!(#name.extend(result.#name);)
            })
            .collect();
        let field_names: Vec<Ident> = fields.iter().map(|f| safe_ident(f.name)).collect();
        let parse_result = quote!(Parsed{ #( #field_names,)* });
        let at_least_one_body = if self.at_least_one.is_some() {
            quote!(
                let ParseOk{result, mut state, ..} = closure::parse(state, tracer, cache)?;
                #assignments
            )
        } else {
            quote!(
                let mut state = state;
            )
        };

        Ok(quote!(
            mod closure{
                use super::*;
                #closure_body
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>
            ) -> ParseResult<'a, Parsed> {
                #declarations
                #at_least_one_body
                loop {
                    match closure::parse(state.clone(), tracer, cache) {
                        Ok(ParseOk{result, state:new_state, ..}) => {
                            #assignments
                            state = new_state;
                        },
                        Err(err) => {
                            state = state.record_error(err);
                            break;
                        }
                    }
                }
                Ok(ParseOk{result:#parse_result, state, farthest_error: None})
            }
        ))
    }

    fn get_fields<'a>(&'a self, grammar: &'a Grammar) -> Result<Vec<FieldDescriptor<'a>>> {
        Ok(set_arity_to_multiple(self.body.get_fields(grammar)?))
    }
}

fn set_arity_to_multiple(fields: Vec<FieldDescriptor>) -> Vec<FieldDescriptor> {
    let mut fields = fields;
    for value in &mut fields {
        value.arity = Arity::Multiple;
    }
    fields
}
