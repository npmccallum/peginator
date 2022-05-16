#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grammar {
    pub rules: Vec<Grammar_rules>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Grammar_rules {
    CharRule(CharRule),
    Rule(Rule),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    pub directives: Vec<DirectiveExpression>,
    pub name: Identifier,
    pub definition: Choice,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharRule {
    pub name: Identifier,
    pub choices: Vec<CharRulePart>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CharRulePart {
    CharRangePart(CharRangePart),
    CharacterRange(CharacterRange),
    Identifier(Identifier),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Choice {
    pub choices: Vec<Sequence>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sequence {
    pub parts: Vec<DelimitedExpression>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Group {
    pub body: Choice,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Optional {
    pub body: Choice,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Closure {
    pub body: Choice,
    pub at_least_one: Option<AtLeastOneMarker>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtLeastOneMarker;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NegativeLookahead {
    pub expr: Box<DelimitedExpression>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterRange {
    pub from: CharRangePart,
    pub to: CharRangePart,
}
pub type CharRangePart = StringItem;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringLiteral {
    pub body: Vec<StringItem>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub name: Option<Identifier>,
    pub boxed: Option<BoxMarker>,
    pub typ: Identifier,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoxMarker;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverrideField {
    pub typ: Identifier,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DelimitedExpression {
    CharacterRange(CharacterRange),
    Closure(Closure),
    EndOfInput(EndOfInput),
    Field(Field),
    Group(Group),
    NegativeLookahead(NegativeLookahead),
    Optional(Optional),
    OverrideField(OverrideField),
    StringLiteral(StringLiteral),
}
pub type Identifier = String;
pub type IdentifierChar = char;
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringItem {
    HexaEscape(HexaEscape),
    SimpleEscape(SimpleEscape),
    Utf8Escape(Utf8Escape),
    char(char),
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimpleEscape {
    SimpleEscapeBackslash(SimpleEscapeBackslash),
    SimpleEscapeCarriageReturn(SimpleEscapeCarriageReturn),
    SimpleEscapeDQuote(SimpleEscapeDQuote),
    SimpleEscapeNewline(SimpleEscapeNewline),
    SimpleEscapeQuote(SimpleEscapeQuote),
    SimpleEscapeTab(SimpleEscapeTab),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleEscapeNewline;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleEscapeCarriageReturn;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleEscapeTab;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleEscapeBackslash;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleEscapeQuote;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleEscapeDQuote;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HexaEscape {
    pub c1: HexChar,
    pub c2: HexChar,
}
pub type HexChar = char;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Utf8Escape {
    pub c1: HexChar,
    pub c2: Option<HexChar>,
    pub c3: Option<HexChar>,
    pub c4: Option<HexChar>,
    pub c5: Option<HexChar>,
    pub c6: Option<HexChar>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirectiveExpression {
    ExportDirective(ExportDirective),
    MemoizeDirective(MemoizeDirective),
    NoSkipWsDirective(NoSkipWsDirective),
    PositionDirective(PositionDirective),
    StringDirective(StringDirective),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringDirective;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharDirective;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoSkipWsDirective;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportDirective;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PositionDirective;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoizeDirective;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndOfInput;
impl peginator_generated::PegParser for Grammar {
    fn parse_advanced<T: peginator_generated::ParseTracer>(
        s: &str,
        settings: &peginator_generated::ParseSettings,
    ) -> Result<Self, peginator_generated::ParseError> {
        Ok(peginator_generated::parse_Grammar(
            peginator_generated::ParseState::new(s, settings),
            T::new(),
            &mut Default::default(),
        )?
        .result)
    }
}
#[allow(non_snake_case, unused_variables, unused_imports, unused_mut)]
mod peginator_generated {
    use super::*;
    use crate::runtime::*;
    pub use crate::runtime::{
        IndentedTracer, ParseError, ParseSettings, ParseState, ParseTracer, PegParser,
    };
    #[derive(Default)]
    pub struct ParseCache<'a> {
        _please_dont_complain: std::marker::PhantomData<&'a ()>,
    }
    mod Grammar_impl {
        use super::*;
        mod part_0 {
            use super::*;
            mod closure {
                use super::*;
                mod part_0 {
                    use super::*;
                    mod choice_0 {
                        use super::*;
                        #[inline(always)]
                        pub fn parse<'a>(
                            state: ParseState<'a>,
                            tracer: impl ParseTracer,
                            cache: &mut ParseCache<'a>,
                        ) -> ParseResult<'a, Parsed> {
                            let state = state.skip_whitespace();
                            let ok_result = parse_Rule(state, tracer, cache)?;
                            Ok(ok_result.map(|result| Parsed {
                                rules: vec![Parsed_rules::Rule(result)],
                            }))
                        }
                        #[derive(Debug, Clone, PartialEq, Eq)]
                        pub struct Parsed {
                            pub rules: Vec<Parsed_rules>,
                        }
                    }
                    mod choice_1 {
                        use super::*;
                        #[inline(always)]
                        pub fn parse<'a>(
                            state: ParseState<'a>,
                            tracer: impl ParseTracer,
                            cache: &mut ParseCache<'a>,
                        ) -> ParseResult<'a, Parsed> {
                            let state = state.skip_whitespace();
                            let ok_result = parse_CharRule(state, tracer, cache)?;
                            Ok(ok_result.map(|result| Parsed {
                                rules: vec![Parsed_rules::CharRule(result)],
                            }))
                        }
                        #[derive(Debug, Clone, PartialEq, Eq)]
                        pub struct Parsed {
                            pub rules: Vec<Parsed_rules>,
                        }
                    }
                    #[inline(always)]
                    pub fn parse<'a>(
                        state: ParseState<'a>,
                        tracer: impl ParseTracer,
                        cache: &mut ParseCache<'a>,
                    ) -> ParseResult<'a, Parsed> {
                        let mut state = state;
                        let mut farthest_error: Option<ParseError> = None;
                        match choice_0::parse(state.clone(), tracer.clone(), cache) {
                            Ok(ok_result) => {
                                return Ok(ok_result.map(|result| Parsed {
                                    rules: result.rules,
                                }))
                            }
                            Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
                        }
                        match choice_1::parse(state.clone(), tracer.clone(), cache) {
                            Ok(ok_result) => {
                                return Ok(ok_result.map(|result| Parsed {
                                    rules: result.rules,
                                }))
                            }
                            Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
                        }
                        Err(farthest_error
                            .unwrap_or_else(|| state.report_error(ParseErrorSpecifics::Other)))
                    }
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct Parsed {
                        pub rules: Vec<Parsed_rules>,
                    }
                }
                mod part_1 {
                    use super::*;
                    #[inline(always)]
                    pub fn parse<'a>(
                        state: ParseState<'a>,
                        tracer: impl ParseTracer,
                        cache: &mut ParseCache<'a>,
                    ) -> ParseResult<'a, Parsed> {
                        let state = state.skip_whitespace();
                        let ok_result = parse_character_literal(state, ';')?;
                        Ok(ok_result.map(|_| Parsed))
                    }
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct Parsed;
                }
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let mut state = state;
                    let mut farthest_error: Option<ParseError> = None;
                    let result = match part_0::parse(state, tracer.clone(), cache) {
                        Ok(ParseOk {
                            result,
                            state: new_state,
                            farthest_error: new_farthest_error,
                        }) => {
                            farthest_error = combine_errors(farthest_error, new_farthest_error);
                            state = new_state;
                            result
                        }
                        Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                    };
                    let mut rules = result.rules;
                    match part_1::parse(state, tracer.clone(), cache) {
                        Ok(ParseOk {
                            result: _,
                            state: new_state,
                            farthest_error: new_farthest_error,
                        }) => {
                            farthest_error = combine_errors(farthest_error, new_farthest_error);
                            state = new_state;
                        }
                        Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                    }
                    Ok(ParseOk {
                        result: Parsed { rules },
                        state,
                        farthest_error,
                    })
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub rules: Vec<Parsed_rules>,
                }
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let mut rules: Vec<Parsed_rules> = Vec::new();
                let mut state = state;
                let mut farthest_error: Option<ParseError> = None;
                loop {
                    match closure::parse(state.clone(), tracer.clone(), cache) {
                        Ok(ParseOk {
                            result,
                            state: new_state,
                            farthest_error: new_farthest_error,
                        }) => {
                            rules.extend(result.rules);
                            state = new_state;
                            farthest_error = combine_errors(farthest_error, new_farthest_error);
                        }
                        Err(err) => {
                            farthest_error = combine_errors(farthest_error, Some(err));
                            break;
                        }
                    }
                }
                Ok(ParseOk {
                    result: Parsed { rules },
                    state,
                    farthest_error,
                })
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub rules: Vec<Parsed_rules>,
            }
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                if state.is_empty() {
                    Ok(ParseOk {
                        result: Parsed,
                        state,
                        farthest_error: None,
                    })
                } else {
                    Err(state.report_error(ParseErrorSpecifics::ExpectedEoi))
                }
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            let result = match part_0::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut rules = result.rules;
            match part_1::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            Ok(ParseOk {
                result: Parsed { rules },
                state,
                farthest_error,
            })
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub rules: Vec<Parsed_rules>,
        }
        use super::Grammar_rules as Parsed_rules;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::Grammar> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|r| super::Grammar { rules: r.rules }))
        }
    }
    #[inline]
    pub(super) fn parse_Grammar<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, Grammar> {
        tracer.run_traced("Grammar", state, |state, tracer| {
            Grammar_impl::rule_parser(state, tracer, cache)
        })
    }
    mod Rule_impl {
        use super::*;
        mod part_0 {
            use super::*;
            mod closure {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let state = state.skip_whitespace();
                    let ok_result = parse_DirectiveExpression(state, tracer, cache)?;
                    Ok(ok_result.map(|result| Parsed {
                        directives: vec![result],
                    }))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub directives: Vec<DirectiveExpression>,
                }
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let mut directives: Vec<DirectiveExpression> = Vec::new();
                let mut state = state;
                let mut farthest_error: Option<ParseError> = None;
                loop {
                    match closure::parse(state.clone(), tracer.clone(), cache) {
                        Ok(ParseOk {
                            result,
                            state: new_state,
                            farthest_error: new_farthest_error,
                        }) => {
                            directives.extend(result.directives);
                            state = new_state;
                            farthest_error = combine_errors(farthest_error, new_farthest_error);
                        }
                        Err(err) => {
                            farthest_error = combine_errors(farthest_error, Some(err));
                            break;
                        }
                    }
                }
                Ok(ParseOk {
                    result: Parsed { directives },
                    state,
                    farthest_error,
                })
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub directives: Vec<DirectiveExpression>,
            }
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_Identifier(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed { name: result }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub name: Identifier,
            }
        }
        mod part_2 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_character_literal(state, '=')?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        mod part_3 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_Choice(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed { definition: result }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub definition: Choice,
            }
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            let result = match part_0::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut directives = result.directives;
            let result = match part_1::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut name = result.name;
            match part_2::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            let result = match part_3::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut definition = result.definition;
            Ok(ParseOk {
                result: Parsed {
                    directives,
                    name,
                    definition,
                },
                state,
                farthest_error,
            })
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub directives: Vec<DirectiveExpression>,
            pub name: Identifier,
            pub definition: Choice,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::Rule> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|r| super::Rule {
                directives: r.directives,
                name: r.name,
                definition: r.definition,
            }))
        }
    }
    #[inline]
    pub(super) fn parse_Rule<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, Rule> {
        tracer.run_traced("Rule", state, |state, tracer| {
            Rule_impl::rule_parser(state, tracer, cache)
        })
    }
    mod CharRule_impl {
        use super::*;
        mod part_0 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_CharDirective(state, tracer, cache)?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_Identifier(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed { name: result }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub name: Identifier,
            }
        }
        mod part_2 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_character_literal(state, '=')?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        mod part_3 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_CharRulePart(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    choices: vec![result],
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub choices: Vec<CharRulePart>,
            }
        }
        mod part_4 {
            use super::*;
            mod closure {
                use super::*;
                mod part_0 {
                    use super::*;
                    #[inline(always)]
                    pub fn parse<'a>(
                        state: ParseState<'a>,
                        tracer: impl ParseTracer,
                        cache: &mut ParseCache<'a>,
                    ) -> ParseResult<'a, Parsed> {
                        let state = state.skip_whitespace();
                        let ok_result = parse_character_literal(state, '|')?;
                        Ok(ok_result.map(|_| Parsed))
                    }
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct Parsed;
                }
                mod part_1 {
                    use super::*;
                    #[inline(always)]
                    pub fn parse<'a>(
                        state: ParseState<'a>,
                        tracer: impl ParseTracer,
                        cache: &mut ParseCache<'a>,
                    ) -> ParseResult<'a, Parsed> {
                        let state = state.skip_whitespace();
                        let ok_result = parse_CharRulePart(state, tracer, cache)?;
                        Ok(ok_result.map(|result| Parsed {
                            choices: vec![result],
                        }))
                    }
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct Parsed {
                        pub choices: Vec<CharRulePart>,
                    }
                }
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let mut state = state;
                    let mut farthest_error: Option<ParseError> = None;
                    match part_0::parse(state, tracer.clone(), cache) {
                        Ok(ParseOk {
                            result: _,
                            state: new_state,
                            farthest_error: new_farthest_error,
                        }) => {
                            farthest_error = combine_errors(farthest_error, new_farthest_error);
                            state = new_state;
                        }
                        Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                    }
                    let result = match part_1::parse(state, tracer.clone(), cache) {
                        Ok(ParseOk {
                            result,
                            state: new_state,
                            farthest_error: new_farthest_error,
                        }) => {
                            farthest_error = combine_errors(farthest_error, new_farthest_error);
                            state = new_state;
                            result
                        }
                        Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                    };
                    let mut choices = result.choices;
                    Ok(ParseOk {
                        result: Parsed { choices },
                        state,
                        farthest_error,
                    })
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub choices: Vec<CharRulePart>,
                }
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let mut choices: Vec<CharRulePart> = Vec::new();
                let mut state = state;
                let mut farthest_error: Option<ParseError> = None;
                loop {
                    match closure::parse(state.clone(), tracer.clone(), cache) {
                        Ok(ParseOk {
                            result,
                            state: new_state,
                            farthest_error: new_farthest_error,
                        }) => {
                            choices.extend(result.choices);
                            state = new_state;
                            farthest_error = combine_errors(farthest_error, new_farthest_error);
                        }
                        Err(err) => {
                            farthest_error = combine_errors(farthest_error, Some(err));
                            break;
                        }
                    }
                }
                Ok(ParseOk {
                    result: Parsed { choices },
                    state,
                    farthest_error,
                })
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub choices: Vec<CharRulePart>,
            }
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            match part_0::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            let result = match part_1::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut name = result.name;
            match part_2::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            let result = match part_3::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut choices = result.choices;
            let result = match part_4::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            choices.extend(result.choices);
            Ok(ParseOk {
                result: Parsed { name, choices },
                state,
                farthest_error,
            })
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub name: Identifier,
            pub choices: Vec<CharRulePart>,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::CharRule> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|r| super::CharRule {
                name: r.name,
                choices: r.choices,
            }))
        }
    }
    #[inline]
    pub(super) fn parse_CharRule<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, CharRule> {
        tracer.run_traced("CharRule", state, |state, tracer| {
            CharRule_impl::rule_parser(state, tracer, cache)
        })
    }
    mod CharRulePart_impl {
        use super::*;
        mod choice_0 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_CharacterRange(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::CharacterRange(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_1 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_CharRangePart(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::CharRangePart(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_2 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_Identifier(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::Identifier(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            match choice_0::parse(state.clone(), tracer.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_1::parse(state.clone(), tracer.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_2::parse(state.clone(), tracer.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            Err(farthest_error.unwrap_or_else(|| state.report_error(ParseErrorSpecifics::Other)))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub _override: Parsed__override,
        }
        use super::CharRulePart as Parsed__override;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::CharRulePart> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|result| result._override))
        }
    }
    #[inline]
    pub(super) fn parse_CharRulePart<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, CharRulePart> {
        tracer.run_traced("CharRulePart", state, |state, tracer| {
            CharRulePart_impl::rule_parser(state, tracer, cache)
        })
    }
    mod Choice_impl {
        use super::*;
        mod part_0 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_Sequence(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    choices: vec![result],
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub choices: Vec<Sequence>,
            }
        }
        mod part_1 {
            use super::*;
            mod closure {
                use super::*;
                mod part_0 {
                    use super::*;
                    #[inline(always)]
                    pub fn parse<'a>(
                        state: ParseState<'a>,
                        tracer: impl ParseTracer,
                        cache: &mut ParseCache<'a>,
                    ) -> ParseResult<'a, Parsed> {
                        let state = state.skip_whitespace();
                        let ok_result = parse_character_literal(state, '|')?;
                        Ok(ok_result.map(|_| Parsed))
                    }
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct Parsed;
                }
                mod part_1 {
                    use super::*;
                    #[inline(always)]
                    pub fn parse<'a>(
                        state: ParseState<'a>,
                        tracer: impl ParseTracer,
                        cache: &mut ParseCache<'a>,
                    ) -> ParseResult<'a, Parsed> {
                        let state = state.skip_whitespace();
                        let ok_result = parse_Sequence(state, tracer, cache)?;
                        Ok(ok_result.map(|result| Parsed {
                            choices: vec![result],
                        }))
                    }
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct Parsed {
                        pub choices: Vec<Sequence>,
                    }
                }
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let mut state = state;
                    let mut farthest_error: Option<ParseError> = None;
                    match part_0::parse(state, tracer.clone(), cache) {
                        Ok(ParseOk {
                            result: _,
                            state: new_state,
                            farthest_error: new_farthest_error,
                        }) => {
                            farthest_error = combine_errors(farthest_error, new_farthest_error);
                            state = new_state;
                        }
                        Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                    }
                    let result = match part_1::parse(state, tracer.clone(), cache) {
                        Ok(ParseOk {
                            result,
                            state: new_state,
                            farthest_error: new_farthest_error,
                        }) => {
                            farthest_error = combine_errors(farthest_error, new_farthest_error);
                            state = new_state;
                            result
                        }
                        Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                    };
                    let mut choices = result.choices;
                    Ok(ParseOk {
                        result: Parsed { choices },
                        state,
                        farthest_error,
                    })
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub choices: Vec<Sequence>,
                }
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let mut choices: Vec<Sequence> = Vec::new();
                let mut state = state;
                let mut farthest_error: Option<ParseError> = None;
                loop {
                    match closure::parse(state.clone(), tracer.clone(), cache) {
                        Ok(ParseOk {
                            result,
                            state: new_state,
                            farthest_error: new_farthest_error,
                        }) => {
                            choices.extend(result.choices);
                            state = new_state;
                            farthest_error = combine_errors(farthest_error, new_farthest_error);
                        }
                        Err(err) => {
                            farthest_error = combine_errors(farthest_error, Some(err));
                            break;
                        }
                    }
                }
                Ok(ParseOk {
                    result: Parsed { choices },
                    state,
                    farthest_error,
                })
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub choices: Vec<Sequence>,
            }
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            let result = match part_0::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut choices = result.choices;
            let result = match part_1::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            choices.extend(result.choices);
            Ok(ParseOk {
                result: Parsed { choices },
                state,
                farthest_error,
            })
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub choices: Vec<Sequence>,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::Choice> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|r| super::Choice { choices: r.choices }))
        }
    }
    #[inline]
    pub(super) fn parse_Choice<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, Choice> {
        tracer.run_traced("Choice", state, |state, tracer| {
            Choice_impl::rule_parser(state, tracer, cache)
        })
    }
    mod Sequence_impl {
        use super::*;
        mod closure {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_DelimitedExpression(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    parts: vec![result],
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub parts: Vec<DelimitedExpression>,
            }
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut parts: Vec<DelimitedExpression> = Vec::new();
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            loop {
                match closure::parse(state.clone(), tracer.clone(), cache) {
                    Ok(ParseOk {
                        result,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        parts.extend(result.parts);
                        state = new_state;
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                    }
                    Err(err) => {
                        farthest_error = combine_errors(farthest_error, Some(err));
                        break;
                    }
                }
            }
            Ok(ParseOk {
                result: Parsed { parts },
                state,
                farthest_error,
            })
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub parts: Vec<DelimitedExpression>,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::Sequence> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|r| super::Sequence { parts: r.parts }))
        }
    }
    #[inline]
    pub(super) fn parse_Sequence<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, Sequence> {
        tracer.run_traced("Sequence", state, |state, tracer| {
            Sequence_impl::rule_parser(state, tracer, cache)
        })
    }
    mod Group_impl {
        use super::*;
        mod part_0 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_character_literal(state, '(')?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_Choice(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed { body: result }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub body: Choice,
            }
        }
        mod part_2 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_character_literal(state, ')')?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            match part_0::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            let result = match part_1::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut body = result.body;
            match part_2::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            Ok(ParseOk {
                result: Parsed { body },
                state,
                farthest_error,
            })
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub body: Choice,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::Group> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|r| super::Group { body: r.body }))
        }
    }
    #[inline]
    pub(super) fn parse_Group<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, Group> {
        tracer.run_traced("Group", state, |state, tracer| {
            Group_impl::rule_parser(state, tracer, cache)
        })
    }
    mod Optional_impl {
        use super::*;
        mod part_0 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_character_literal(state, '[')?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_Choice(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed { body: result }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub body: Choice,
            }
        }
        mod part_2 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_character_literal(state, ']')?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            match part_0::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            let result = match part_1::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut body = result.body;
            match part_2::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            Ok(ParseOk {
                result: Parsed { body },
                state,
                farthest_error,
            })
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub body: Choice,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::Optional> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|r| super::Optional { body: r.body }))
        }
    }
    #[inline]
    pub(super) fn parse_Optional<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, Optional> {
        tracer.run_traced("Optional", state, |state, tracer| {
            Optional_impl::rule_parser(state, tracer, cache)
        })
    }
    mod Closure_impl {
        use super::*;
        mod part_0 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_character_literal(state, '{')?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_Choice(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed { body: result }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub body: Choice,
            }
        }
        mod part_2 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_character_literal(state, '}')?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        mod part_3 {
            use super::*;
            mod optional {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let state = state.skip_whitespace();
                    let ok_result = parse_AtLeastOneMarker(state, tracer, cache)?;
                    Ok(ok_result.map(|result| Parsed {
                        at_least_one: Some(result),
                    }))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub at_least_one: Option<AtLeastOneMarker>,
                }
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                match optional::parse(state.clone(), tracer, cache) {
                    Ok(ok_result) => Ok(ok_result.map(|result| Parsed {
                        at_least_one: result.at_least_one,
                    })),
                    Err(err) => Ok(ParseOk {
                        result: Parsed {
                            at_least_one: Default::default(),
                        },
                        state,
                        farthest_error: Some(err),
                    }),
                }
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub at_least_one: Option<AtLeastOneMarker>,
            }
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            match part_0::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            let result = match part_1::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut body = result.body;
            match part_2::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            let result = match part_3::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut at_least_one = result.at_least_one;
            Ok(ParseOk {
                result: Parsed { body, at_least_one },
                state,
                farthest_error,
            })
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub body: Choice,
            pub at_least_one: Option<AtLeastOneMarker>,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::Closure> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|r| super::Closure {
                body: r.body,
                at_least_one: r.at_least_one,
            }))
        }
    }
    #[inline]
    pub(super) fn parse_Closure<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, Closure> {
        tracer.run_traced("Closure", state, |state, tracer| {
            Closure_impl::rule_parser(state, tracer, cache)
        })
    }
    mod AtLeastOneMarker_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let state = state.skip_whitespace();
            let ok_result = parse_character_literal(state, '+')?;
            Ok(ok_result.map(|_| Parsed))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::AtLeastOneMarker> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|r| super::AtLeastOneMarker {}))
        }
    }
    #[inline]
    pub(super) fn parse_AtLeastOneMarker<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, AtLeastOneMarker> {
        tracer.run_traced("AtLeastOneMarker", state, |state, tracer| {
            AtLeastOneMarker_impl::rule_parser(state, tracer, cache)
        })
    }
    mod NegativeLookahead_impl {
        use super::*;
        mod part_0 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_character_literal(state, '!')?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_DelimitedExpression(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    expr: Box::new(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub expr: Box<DelimitedExpression>,
            }
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            match part_0::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            let result = match part_1::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut expr = result.expr;
            Ok(ParseOk {
                result: Parsed { expr },
                state,
                farthest_error,
            })
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub expr: Box<DelimitedExpression>,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::NegativeLookahead> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|r| super::NegativeLookahead { expr: r.expr }))
        }
    }
    #[inline]
    pub(super) fn parse_NegativeLookahead<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, NegativeLookahead> {
        tracer.run_traced("NegativeLookahead", state, |state, tracer| {
            NegativeLookahead_impl::rule_parser(state, tracer, cache)
        })
    }
    mod CharacterRange_impl {
        use super::*;
        mod part_0 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_CharRangePart(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed { from: result }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub from: CharRangePart,
            }
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_string_literal(state, "..")?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        mod part_2 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_CharRangePart(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed { to: result }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub to: CharRangePart,
            }
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            let result = match part_0::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut from = result.from;
            match part_1::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            let result = match part_2::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut to = result.to;
            Ok(ParseOk {
                result: Parsed { from, to },
                state,
                farthest_error,
            })
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub from: CharRangePart,
            pub to: CharRangePart,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::CharacterRange> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|r| super::CharacterRange {
                from: r.from,
                to: r.to,
            }))
        }
    }
    #[inline]
    pub(super) fn parse_CharacterRange<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, CharacterRange> {
        tracer.run_traced("CharacterRange", state, |state, tracer| {
            CharacterRange_impl::rule_parser(state, tracer, cache)
        })
    }
    mod CharRangePart_impl {
        use super::*;
        mod part_0 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let ok_result = parse_character_literal(state, '\'')?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let ok_result = parse_StringItem(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed { _override: result }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: StringItem,
            }
        }
        mod part_2 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let ok_result = parse_character_literal(state, '\'')?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            match part_0::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            let result = match part_1::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut _override = result._override;
            match part_2::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            Ok(ParseOk {
                result: Parsed { _override },
                state,
                farthest_error,
            })
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub _override: StringItem,
        }
        use super::CharRangePart as Parsed__override;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::CharRangePart> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|result| result._override))
        }
    }
    #[inline]
    pub(super) fn parse_CharRangePart<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, CharRangePart> {
        tracer.run_traced("CharRangePart", state, |state, tracer| {
            CharRangePart_impl::rule_parser(state, tracer, cache)
        })
    }
    mod StringLiteral_impl {
        use super::*;
        mod choice_0 {
            use super::*;
            mod part_0 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_character_literal(state, '"')?;
                    Ok(ok_result.map(|_| Parsed))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed;
            }
            mod part_1 {
                use super::*;
                mod closure {
                    use super::*;
                    mod part_0 {
                        use super::*;
                        mod negative_lookahead {
                            use super::*;
                            #[inline(always)]
                            pub fn parse<'a>(
                                state: ParseState<'a>,
                                tracer: impl ParseTracer,
                                cache: &mut ParseCache<'a>,
                            ) -> ParseResult<'a, Parsed> {
                                let ok_result = parse_character_literal(state, '"')?;
                                Ok(ok_result.map(|_| Parsed))
                            }
                            #[derive(Debug, Clone, PartialEq, Eq)]
                            pub struct Parsed;
                        }
                        #[inline(always)]
                        pub fn parse<'a>(
                            state: ParseState<'a>,
                            tracer: impl ParseTracer,
                            cache: &mut ParseCache<'a>,
                        ) -> ParseResult<'a, Parsed> {
                            match negative_lookahead::parse(state.clone(), tracer, cache) {
                                Ok(_) => Err(state
                                    .report_error(ParseErrorSpecifics::NegativeLookaheadFailed)),
                                Err(_) => Ok(ParseOk {
                                    result: Parsed,
                                    state,
                                    farthest_error: None,
                                }),
                            }
                        }
                        #[derive(Debug, Clone, PartialEq, Eq)]
                        pub struct Parsed;
                    }
                    mod part_1 {
                        use super::*;
                        #[inline(always)]
                        pub fn parse<'a>(
                            state: ParseState<'a>,
                            tracer: impl ParseTracer,
                            cache: &mut ParseCache<'a>,
                        ) -> ParseResult<'a, Parsed> {
                            let ok_result = parse_StringItem(state, tracer, cache)?;
                            Ok(ok_result.map(|result| Parsed { body: vec![result] }))
                        }
                        #[derive(Debug, Clone, PartialEq, Eq)]
                        pub struct Parsed {
                            pub body: Vec<StringItem>,
                        }
                    }
                    #[inline(always)]
                    pub fn parse<'a>(
                        state: ParseState<'a>,
                        tracer: impl ParseTracer,
                        cache: &mut ParseCache<'a>,
                    ) -> ParseResult<'a, Parsed> {
                        let mut state = state;
                        let mut farthest_error: Option<ParseError> = None;
                        match part_0::parse(state, tracer.clone(), cache) {
                            Ok(ParseOk {
                                result: _,
                                state: new_state,
                                farthest_error: new_farthest_error,
                            }) => {
                                farthest_error = combine_errors(farthest_error, new_farthest_error);
                                state = new_state;
                            }
                            Err(err) => {
                                return Err(combine_errors(farthest_error, Some(err)).unwrap())
                            }
                        }
                        let result = match part_1::parse(state, tracer.clone(), cache) {
                            Ok(ParseOk {
                                result,
                                state: new_state,
                                farthest_error: new_farthest_error,
                            }) => {
                                farthest_error = combine_errors(farthest_error, new_farthest_error);
                                state = new_state;
                                result
                            }
                            Err(err) => {
                                return Err(combine_errors(farthest_error, Some(err)).unwrap())
                            }
                        };
                        let mut body = result.body;
                        Ok(ParseOk {
                            result: Parsed { body },
                            state,
                            farthest_error,
                        })
                    }
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct Parsed {
                        pub body: Vec<StringItem>,
                    }
                }
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let mut body: Vec<StringItem> = Vec::new();
                    let mut state = state;
                    let mut farthest_error: Option<ParseError> = None;
                    loop {
                        match closure::parse(state.clone(), tracer.clone(), cache) {
                            Ok(ParseOk {
                                result,
                                state: new_state,
                                farthest_error: new_farthest_error,
                            }) => {
                                body.extend(result.body);
                                state = new_state;
                                farthest_error = combine_errors(farthest_error, new_farthest_error);
                            }
                            Err(err) => {
                                farthest_error = combine_errors(farthest_error, Some(err));
                                break;
                            }
                        }
                    }
                    Ok(ParseOk {
                        result: Parsed { body },
                        state,
                        farthest_error,
                    })
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub body: Vec<StringItem>,
                }
            }
            mod part_2 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_character_literal(state, '"')?;
                    Ok(ok_result.map(|_| Parsed))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed;
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let mut state = state;
                let mut farthest_error: Option<ParseError> = None;
                match part_0::parse(state, tracer.clone(), cache) {
                    Ok(ParseOk {
                        result: _,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                        state = new_state;
                    }
                    Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                }
                let result = match part_1::parse(state, tracer.clone(), cache) {
                    Ok(ParseOk {
                        result,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                        state = new_state;
                        result
                    }
                    Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                };
                let mut body = result.body;
                match part_2::parse(state, tracer.clone(), cache) {
                    Ok(ParseOk {
                        result: _,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                        state = new_state;
                    }
                    Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                }
                Ok(ParseOk {
                    result: Parsed { body },
                    state,
                    farthest_error,
                })
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub body: Vec<StringItem>,
            }
        }
        mod choice_1 {
            use super::*;
            mod part_0 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_character_literal(state, '\'')?;
                    Ok(ok_result.map(|_| Parsed))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed;
            }
            mod part_1 {
                use super::*;
                mod closure {
                    use super::*;
                    mod part_0 {
                        use super::*;
                        mod negative_lookahead {
                            use super::*;
                            #[inline(always)]
                            pub fn parse<'a>(
                                state: ParseState<'a>,
                                tracer: impl ParseTracer,
                                cache: &mut ParseCache<'a>,
                            ) -> ParseResult<'a, Parsed> {
                                let ok_result = parse_character_literal(state, '\'')?;
                                Ok(ok_result.map(|_| Parsed))
                            }
                            #[derive(Debug, Clone, PartialEq, Eq)]
                            pub struct Parsed;
                        }
                        #[inline(always)]
                        pub fn parse<'a>(
                            state: ParseState<'a>,
                            tracer: impl ParseTracer,
                            cache: &mut ParseCache<'a>,
                        ) -> ParseResult<'a, Parsed> {
                            match negative_lookahead::parse(state.clone(), tracer, cache) {
                                Ok(_) => Err(state
                                    .report_error(ParseErrorSpecifics::NegativeLookaheadFailed)),
                                Err(_) => Ok(ParseOk {
                                    result: Parsed,
                                    state,
                                    farthest_error: None,
                                }),
                            }
                        }
                        #[derive(Debug, Clone, PartialEq, Eq)]
                        pub struct Parsed;
                    }
                    mod part_1 {
                        use super::*;
                        #[inline(always)]
                        pub fn parse<'a>(
                            state: ParseState<'a>,
                            tracer: impl ParseTracer,
                            cache: &mut ParseCache<'a>,
                        ) -> ParseResult<'a, Parsed> {
                            let ok_result = parse_StringItem(state, tracer, cache)?;
                            Ok(ok_result.map(|result| Parsed { body: vec![result] }))
                        }
                        #[derive(Debug, Clone, PartialEq, Eq)]
                        pub struct Parsed {
                            pub body: Vec<StringItem>,
                        }
                    }
                    #[inline(always)]
                    pub fn parse<'a>(
                        state: ParseState<'a>,
                        tracer: impl ParseTracer,
                        cache: &mut ParseCache<'a>,
                    ) -> ParseResult<'a, Parsed> {
                        let mut state = state;
                        let mut farthest_error: Option<ParseError> = None;
                        match part_0::parse(state, tracer.clone(), cache) {
                            Ok(ParseOk {
                                result: _,
                                state: new_state,
                                farthest_error: new_farthest_error,
                            }) => {
                                farthest_error = combine_errors(farthest_error, new_farthest_error);
                                state = new_state;
                            }
                            Err(err) => {
                                return Err(combine_errors(farthest_error, Some(err)).unwrap())
                            }
                        }
                        let result = match part_1::parse(state, tracer.clone(), cache) {
                            Ok(ParseOk {
                                result,
                                state: new_state,
                                farthest_error: new_farthest_error,
                            }) => {
                                farthest_error = combine_errors(farthest_error, new_farthest_error);
                                state = new_state;
                                result
                            }
                            Err(err) => {
                                return Err(combine_errors(farthest_error, Some(err)).unwrap())
                            }
                        };
                        let mut body = result.body;
                        Ok(ParseOk {
                            result: Parsed { body },
                            state,
                            farthest_error,
                        })
                    }
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct Parsed {
                        pub body: Vec<StringItem>,
                    }
                }
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let mut body: Vec<StringItem> = Vec::new();
                    let mut state = state;
                    let mut farthest_error: Option<ParseError> = None;
                    loop {
                        match closure::parse(state.clone(), tracer.clone(), cache) {
                            Ok(ParseOk {
                                result,
                                state: new_state,
                                farthest_error: new_farthest_error,
                            }) => {
                                body.extend(result.body);
                                state = new_state;
                                farthest_error = combine_errors(farthest_error, new_farthest_error);
                            }
                            Err(err) => {
                                farthest_error = combine_errors(farthest_error, Some(err));
                                break;
                            }
                        }
                    }
                    Ok(ParseOk {
                        result: Parsed { body },
                        state,
                        farthest_error,
                    })
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub body: Vec<StringItem>,
                }
            }
            mod part_2 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_character_literal(state, '\'')?;
                    Ok(ok_result.map(|_| Parsed))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed;
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let mut state = state;
                let mut farthest_error: Option<ParseError> = None;
                match part_0::parse(state, tracer.clone(), cache) {
                    Ok(ParseOk {
                        result: _,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                        state = new_state;
                    }
                    Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                }
                let result = match part_1::parse(state, tracer.clone(), cache) {
                    Ok(ParseOk {
                        result,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                        state = new_state;
                        result
                    }
                    Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                };
                let mut body = result.body;
                match part_2::parse(state, tracer.clone(), cache) {
                    Ok(ParseOk {
                        result: _,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                        state = new_state;
                    }
                    Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                }
                Ok(ParseOk {
                    result: Parsed { body },
                    state,
                    farthest_error,
                })
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub body: Vec<StringItem>,
            }
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            match choice_0::parse(state.clone(), tracer.clone(), cache) {
                Ok(ok_result) => return Ok(ok_result.map(|result| Parsed { body: result.body })),
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_1::parse(state.clone(), tracer.clone(), cache) {
                Ok(ok_result) => return Ok(ok_result.map(|result| Parsed { body: result.body })),
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            Err(farthest_error.unwrap_or_else(|| state.report_error(ParseErrorSpecifics::Other)))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub body: Vec<StringItem>,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::StringLiteral> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|r| super::StringLiteral { body: r.body }))
        }
    }
    #[inline]
    pub(super) fn parse_StringLiteral<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, StringLiteral> {
        tracer.run_traced("StringLiteral", state, |state, tracer| {
            StringLiteral_impl::rule_parser(state, tracer, cache)
        })
    }
    mod Field_impl {
        use super::*;
        mod part_0 {
            use super::*;
            mod optional {
                use super::*;
                mod part_0 {
                    use super::*;
                    #[inline(always)]
                    pub fn parse<'a>(
                        state: ParseState<'a>,
                        tracer: impl ParseTracer,
                        cache: &mut ParseCache<'a>,
                    ) -> ParseResult<'a, Parsed> {
                        let state = state.skip_whitespace();
                        let ok_result = parse_Identifier(state, tracer, cache)?;
                        Ok(ok_result.map(|result| Parsed { name: Some(result) }))
                    }
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct Parsed {
                        pub name: Option<Identifier>,
                    }
                }
                mod part_1 {
                    use super::*;
                    #[inline(always)]
                    pub fn parse<'a>(
                        state: ParseState<'a>,
                        tracer: impl ParseTracer,
                        cache: &mut ParseCache<'a>,
                    ) -> ParseResult<'a, Parsed> {
                        let state = state.skip_whitespace();
                        let ok_result = parse_character_literal(state, ':')?;
                        Ok(ok_result.map(|_| Parsed))
                    }
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct Parsed;
                }
                mod part_2 {
                    use super::*;
                    mod optional {
                        use super::*;
                        #[inline(always)]
                        pub fn parse<'a>(
                            state: ParseState<'a>,
                            tracer: impl ParseTracer,
                            cache: &mut ParseCache<'a>,
                        ) -> ParseResult<'a, Parsed> {
                            let state = state.skip_whitespace();
                            let ok_result = parse_BoxMarker(state, tracer, cache)?;
                            Ok(ok_result.map(|result| Parsed {
                                boxed: Some(result),
                            }))
                        }
                        #[derive(Debug, Clone, PartialEq, Eq)]
                        pub struct Parsed {
                            pub boxed: Option<BoxMarker>,
                        }
                    }
                    #[inline(always)]
                    pub fn parse<'a>(
                        state: ParseState<'a>,
                        tracer: impl ParseTracer,
                        cache: &mut ParseCache<'a>,
                    ) -> ParseResult<'a, Parsed> {
                        match optional::parse(state.clone(), tracer, cache) {
                            Ok(ok_result) => Ok(ok_result.map(|result| Parsed {
                                boxed: result.boxed,
                            })),
                            Err(err) => Ok(ParseOk {
                                result: Parsed {
                                    boxed: Default::default(),
                                },
                                state,
                                farthest_error: Some(err),
                            }),
                        }
                    }
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct Parsed {
                        pub boxed: Option<BoxMarker>,
                    }
                }
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let mut state = state;
                    let mut farthest_error: Option<ParseError> = None;
                    let result = match part_0::parse(state, tracer.clone(), cache) {
                        Ok(ParseOk {
                            result,
                            state: new_state,
                            farthest_error: new_farthest_error,
                        }) => {
                            farthest_error = combine_errors(farthest_error, new_farthest_error);
                            state = new_state;
                            result
                        }
                        Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                    };
                    let mut name = result.name;
                    match part_1::parse(state, tracer.clone(), cache) {
                        Ok(ParseOk {
                            result: _,
                            state: new_state,
                            farthest_error: new_farthest_error,
                        }) => {
                            farthest_error = combine_errors(farthest_error, new_farthest_error);
                            state = new_state;
                        }
                        Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                    }
                    let result = match part_2::parse(state, tracer.clone(), cache) {
                        Ok(ParseOk {
                            result,
                            state: new_state,
                            farthest_error: new_farthest_error,
                        }) => {
                            farthest_error = combine_errors(farthest_error, new_farthest_error);
                            state = new_state;
                            result
                        }
                        Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                    };
                    let mut boxed = result.boxed;
                    Ok(ParseOk {
                        result: Parsed { name, boxed },
                        state,
                        farthest_error,
                    })
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub name: Option<Identifier>,
                    pub boxed: Option<BoxMarker>,
                }
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                match optional::parse(state.clone(), tracer, cache) {
                    Ok(ok_result) => Ok(ok_result.map(|result| Parsed {
                        name: result.name,
                        boxed: result.boxed,
                    })),
                    Err(err) => Ok(ParseOk {
                        result: Parsed {
                            name: Default::default(),
                            boxed: Default::default(),
                        },
                        state,
                        farthest_error: Some(err),
                    }),
                }
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub name: Option<Identifier>,
                pub boxed: Option<BoxMarker>,
            }
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_Identifier(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed { typ: result }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub typ: Identifier,
            }
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            let result = match part_0::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut name = result.name;
            let mut boxed = result.boxed;
            let result = match part_1::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut typ = result.typ;
            Ok(ParseOk {
                result: Parsed { name, boxed, typ },
                state,
                farthest_error,
            })
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub name: Option<Identifier>,
            pub boxed: Option<BoxMarker>,
            pub typ: Identifier,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::Field> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|r| super::Field {
                name: r.name,
                boxed: r.boxed,
                typ: r.typ,
            }))
        }
    }
    #[inline]
    pub(super) fn parse_Field<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, Field> {
        tracer.run_traced("Field", state, |state, tracer| {
            Field_impl::rule_parser(state, tracer, cache)
        })
    }
    mod BoxMarker_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let state = state.skip_whitespace();
            let ok_result = parse_character_literal(state, '*')?;
            Ok(ok_result.map(|_| Parsed))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::BoxMarker> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|r| super::BoxMarker {}))
        }
    }
    #[inline]
    pub(super) fn parse_BoxMarker<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, BoxMarker> {
        tracer.run_traced("BoxMarker", state, |state, tracer| {
            BoxMarker_impl::rule_parser(state, tracer, cache)
        })
    }
    mod OverrideField_impl {
        use super::*;
        mod part_0 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_character_literal(state, '@')?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_character_literal(state, ':')?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        mod part_2 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_Identifier(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed { typ: result }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub typ: Identifier,
            }
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            match part_0::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            match part_1::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            let result = match part_2::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut typ = result.typ;
            Ok(ParseOk {
                result: Parsed { typ },
                state,
                farthest_error,
            })
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub typ: Identifier,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::OverrideField> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|r| super::OverrideField { typ: r.typ }))
        }
    }
    #[inline]
    pub(super) fn parse_OverrideField<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, OverrideField> {
        tracer.run_traced("OverrideField", state, |state, tracer| {
            OverrideField_impl::rule_parser(state, tracer, cache)
        })
    }
    mod DelimitedExpression_impl {
        use super::*;
        mod choice_0 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_Group(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::Group(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_1 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_Optional(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::Optional(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_2 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_Closure(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::Closure(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_3 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_NegativeLookahead(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::NegativeLookahead(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_4 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_CharacterRange(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::CharacterRange(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_5 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_StringLiteral(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::StringLiteral(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_6 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_EndOfInput(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::EndOfInput(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_7 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_OverrideField(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::OverrideField(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_8 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_Field(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::Field(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            match choice_0::parse(state.clone(), tracer.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_1::parse(state.clone(), tracer.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_2::parse(state.clone(), tracer.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_3::parse(state.clone(), tracer.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_4::parse(state.clone(), tracer.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_5::parse(state.clone(), tracer.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_6::parse(state.clone(), tracer.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_7::parse(state.clone(), tracer.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_8::parse(state.clone(), tracer.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            Err(farthest_error.unwrap_or_else(|| state.report_error(ParseErrorSpecifics::Other)))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub _override: Parsed__override,
        }
        use super::DelimitedExpression as Parsed__override;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::DelimitedExpression> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|result| result._override))
        }
    }
    #[inline]
    pub(super) fn parse_DelimitedExpression<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, DelimitedExpression> {
        tracer.run_traced("DelimitedExpression", state, |state, tracer| {
            DelimitedExpression_impl::rule_parser(state, tracer, cache)
        })
    }
    mod Identifier_impl {
        use super::*;
        mod closure {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let ok_result = parse_IdentifierChar(state, tracer, cache)?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let ParseOk {
                result,
                mut state,
                mut farthest_error,
            } = closure::parse(state, tracer.clone(), cache)?;
            loop {
                match closure::parse(state.clone(), tracer.clone(), cache) {
                    Ok(ParseOk {
                        result,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        state = new_state;
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                    }
                    Err(err) => {
                        farthest_error = combine_errors(farthest_error, Some(err));
                        break;
                    }
                }
            }
            Ok(ParseOk {
                result: Parsed {},
                state,
                farthest_error,
            })
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, String> {
            let ok_result = parse(state.clone(), tracer, cache)?;
            Ok(ok_result.map_with_state(|_, new_state| state.slice_until(&new_state).to_string()))
        }
    }
    #[inline]
    pub(super) fn parse_Identifier<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, Identifier> {
        tracer.run_traced("Identifier", state, |state, tracer| {
            Identifier_impl::rule_parser(state, tracer, cache)
        })
    }
    #[inline]
    pub(super) fn parse_IdentifierChar<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, IdentifierChar> {
        if let Ok(result) = parse_character_range(state.clone(), 'a', 'z') {
            return Ok(result);
        }
        if let Ok(result) = parse_character_range(state.clone(), 'A', 'Z') {
            return Ok(result);
        }
        if let Ok(result) = parse_character_range(state.clone(), '0', '9') {
            return Ok(result);
        }
        if let Ok(result) = parse_character_literal(state.clone(), '_') {
            return Ok(result);
        }
        Err(
            state.report_error(ParseErrorSpecifics::ExpectedCharacterClass {
                name: "IdentifierChar",
            }),
        )
    }
    mod StringItem_impl {
        use super::*;
        mod choice_0 {
            use super::*;
            mod part_0 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_character_literal(state, '\\')?;
                    Ok(ok_result.map(|_| Parsed))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed;
            }
            mod part_1 {
                use super::*;
                mod choice_0 {
                    use super::*;
                    #[inline(always)]
                    pub fn parse<'a>(
                        state: ParseState<'a>,
                        tracer: impl ParseTracer,
                        cache: &mut ParseCache<'a>,
                    ) -> ParseResult<'a, Parsed> {
                        let ok_result = parse_SimpleEscape(state, tracer, cache)?;
                        Ok(ok_result.map(|result| Parsed {
                            _override: Parsed__override::SimpleEscape(result),
                        }))
                    }
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct Parsed {
                        pub _override: Parsed__override,
                    }
                }
                mod choice_1 {
                    use super::*;
                    #[inline(always)]
                    pub fn parse<'a>(
                        state: ParseState<'a>,
                        tracer: impl ParseTracer,
                        cache: &mut ParseCache<'a>,
                    ) -> ParseResult<'a, Parsed> {
                        let ok_result = parse_HexaEscape(state, tracer, cache)?;
                        Ok(ok_result.map(|result| Parsed {
                            _override: Parsed__override::HexaEscape(result),
                        }))
                    }
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct Parsed {
                        pub _override: Parsed__override,
                    }
                }
                mod choice_2 {
                    use super::*;
                    #[inline(always)]
                    pub fn parse<'a>(
                        state: ParseState<'a>,
                        tracer: impl ParseTracer,
                        cache: &mut ParseCache<'a>,
                    ) -> ParseResult<'a, Parsed> {
                        let ok_result = parse_Utf8Escape(state, tracer, cache)?;
                        Ok(ok_result.map(|result| Parsed {
                            _override: Parsed__override::Utf8Escape(result),
                        }))
                    }
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct Parsed {
                        pub _override: Parsed__override,
                    }
                }
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let mut state = state;
                    let mut farthest_error: Option<ParseError> = None;
                    match choice_0::parse(state.clone(), tracer.clone(), cache) {
                        Ok(ok_result) => {
                            return Ok(ok_result.map(|result| Parsed {
                                _override: result._override,
                            }))
                        }
                        Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
                    }
                    match choice_1::parse(state.clone(), tracer.clone(), cache) {
                        Ok(ok_result) => {
                            return Ok(ok_result.map(|result| Parsed {
                                _override: result._override,
                            }))
                        }
                        Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
                    }
                    match choice_2::parse(state.clone(), tracer.clone(), cache) {
                        Ok(ok_result) => {
                            return Ok(ok_result.map(|result| Parsed {
                                _override: result._override,
                            }))
                        }
                        Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
                    }
                    Err(farthest_error
                        .unwrap_or_else(|| state.report_error(ParseErrorSpecifics::Other)))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub _override: Parsed__override,
                }
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let mut state = state;
                let mut farthest_error: Option<ParseError> = None;
                match part_0::parse(state, tracer.clone(), cache) {
                    Ok(ParseOk {
                        result: _,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                        state = new_state;
                    }
                    Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                }
                let result = match part_1::parse(state, tracer.clone(), cache) {
                    Ok(ParseOk {
                        result,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                        state = new_state;
                        result
                    }
                    Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                };
                let mut _override = result._override;
                Ok(ParseOk {
                    result: Parsed { _override },
                    state,
                    farthest_error,
                })
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_1 {
            use super::*;
            mod part_0 {
                use super::*;
                mod negative_lookahead {
                    use super::*;
                    #[inline(always)]
                    pub fn parse<'a>(
                        state: ParseState<'a>,
                        tracer: impl ParseTracer,
                        cache: &mut ParseCache<'a>,
                    ) -> ParseResult<'a, Parsed> {
                        let ok_result = parse_character_literal(state, '\\')?;
                        Ok(ok_result.map(|_| Parsed))
                    }
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct Parsed;
                }
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    match negative_lookahead::parse(state.clone(), tracer, cache) {
                        Ok(_) => {
                            Err(state.report_error(ParseErrorSpecifics::NegativeLookaheadFailed))
                        }
                        Err(_) => Ok(ParseOk {
                            result: Parsed,
                            state,
                            farthest_error: None,
                        }),
                    }
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed;
            }
            mod part_1 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_char(state, tracer, cache)?;
                    Ok(ok_result.map(|result| Parsed {
                        _override: Parsed__override::char(result),
                    }))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub _override: Parsed__override,
                }
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let mut state = state;
                let mut farthest_error: Option<ParseError> = None;
                match part_0::parse(state, tracer.clone(), cache) {
                    Ok(ParseOk {
                        result: _,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                        state = new_state;
                    }
                    Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                }
                let result = match part_1::parse(state, tracer.clone(), cache) {
                    Ok(ParseOk {
                        result,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                        state = new_state;
                        result
                    }
                    Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                };
                let mut _override = result._override;
                Ok(ParseOk {
                    result: Parsed { _override },
                    state,
                    farthest_error,
                })
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            match choice_0::parse(state.clone(), tracer.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_1::parse(state.clone(), tracer.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            Err(farthest_error.unwrap_or_else(|| state.report_error(ParseErrorSpecifics::Other)))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub _override: Parsed__override,
        }
        use super::StringItem as Parsed__override;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::StringItem> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|result| result._override))
        }
    }
    #[inline]
    pub(super) fn parse_StringItem<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, StringItem> {
        tracer.run_traced("StringItem", state, |state, tracer| {
            StringItem_impl::rule_parser(state, tracer, cache)
        })
    }
    mod SimpleEscape_impl {
        use super::*;
        mod choice_0 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let ok_result = parse_SimpleEscapeNewline(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::SimpleEscapeNewline(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_1 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let ok_result = parse_SimpleEscapeCarriageReturn(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::SimpleEscapeCarriageReturn(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_2 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let ok_result = parse_SimpleEscapeTab(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::SimpleEscapeTab(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_3 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let ok_result = parse_SimpleEscapeBackslash(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::SimpleEscapeBackslash(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_4 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let ok_result = parse_SimpleEscapeQuote(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::SimpleEscapeQuote(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_5 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let ok_result = parse_SimpleEscapeDQuote(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::SimpleEscapeDQuote(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            match choice_0::parse(state.clone(), tracer.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_1::parse(state.clone(), tracer.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_2::parse(state.clone(), tracer.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_3::parse(state.clone(), tracer.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_4::parse(state.clone(), tracer.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_5::parse(state.clone(), tracer.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            Err(farthest_error.unwrap_or_else(|| state.report_error(ParseErrorSpecifics::Other)))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub _override: Parsed__override,
        }
        use super::SimpleEscape as Parsed__override;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::SimpleEscape> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|result| result._override))
        }
    }
    #[inline]
    pub(super) fn parse_SimpleEscape<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, SimpleEscape> {
        tracer.run_traced("SimpleEscape", state, |state, tracer| {
            SimpleEscape_impl::rule_parser(state, tracer, cache)
        })
    }
    mod SimpleEscapeNewline_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let ok_result = parse_character_literal(state, 'n')?;
            Ok(ok_result.map(|_| Parsed))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::SimpleEscapeNewline> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|r| super::SimpleEscapeNewline {}))
        }
    }
    #[inline]
    pub(super) fn parse_SimpleEscapeNewline<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, SimpleEscapeNewline> {
        tracer.run_traced("SimpleEscapeNewline", state, |state, tracer| {
            SimpleEscapeNewline_impl::rule_parser(state, tracer, cache)
        })
    }
    mod SimpleEscapeCarriageReturn_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let ok_result = parse_character_literal(state, 'r')?;
            Ok(ok_result.map(|_| Parsed))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::SimpleEscapeCarriageReturn> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|r| super::SimpleEscapeCarriageReturn {}))
        }
    }
    #[inline]
    pub(super) fn parse_SimpleEscapeCarriageReturn<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, SimpleEscapeCarriageReturn> {
        tracer.run_traced("SimpleEscapeCarriageReturn", state, |state, tracer| {
            SimpleEscapeCarriageReturn_impl::rule_parser(state, tracer, cache)
        })
    }
    mod SimpleEscapeTab_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let ok_result = parse_character_literal(state, 't')?;
            Ok(ok_result.map(|_| Parsed))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::SimpleEscapeTab> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|r| super::SimpleEscapeTab {}))
        }
    }
    #[inline]
    pub(super) fn parse_SimpleEscapeTab<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, SimpleEscapeTab> {
        tracer.run_traced("SimpleEscapeTab", state, |state, tracer| {
            SimpleEscapeTab_impl::rule_parser(state, tracer, cache)
        })
    }
    mod SimpleEscapeBackslash_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let ok_result = parse_character_literal(state, '\\')?;
            Ok(ok_result.map(|_| Parsed))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::SimpleEscapeBackslash> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|r| super::SimpleEscapeBackslash {}))
        }
    }
    #[inline]
    pub(super) fn parse_SimpleEscapeBackslash<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, SimpleEscapeBackslash> {
        tracer.run_traced("SimpleEscapeBackslash", state, |state, tracer| {
            SimpleEscapeBackslash_impl::rule_parser(state, tracer, cache)
        })
    }
    mod SimpleEscapeQuote_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let ok_result = parse_character_literal(state, '\'')?;
            Ok(ok_result.map(|_| Parsed))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::SimpleEscapeQuote> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|r| super::SimpleEscapeQuote {}))
        }
    }
    #[inline]
    pub(super) fn parse_SimpleEscapeQuote<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, SimpleEscapeQuote> {
        tracer.run_traced("SimpleEscapeQuote", state, |state, tracer| {
            SimpleEscapeQuote_impl::rule_parser(state, tracer, cache)
        })
    }
    mod SimpleEscapeDQuote_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let ok_result = parse_character_literal(state, '"')?;
            Ok(ok_result.map(|_| Parsed))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::SimpleEscapeDQuote> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|r| super::SimpleEscapeDQuote {}))
        }
    }
    #[inline]
    pub(super) fn parse_SimpleEscapeDQuote<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, SimpleEscapeDQuote> {
        tracer.run_traced("SimpleEscapeDQuote", state, |state, tracer| {
            SimpleEscapeDQuote_impl::rule_parser(state, tracer, cache)
        })
    }
    mod HexaEscape_impl {
        use super::*;
        mod part_0 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let ok_result = parse_character_literal(state, 'x')?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let ok_result = parse_HexChar(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed { c1: result }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub c1: HexChar,
            }
        }
        mod part_2 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let ok_result = parse_HexChar(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed { c2: result }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub c2: HexChar,
            }
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            match part_0::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            let result = match part_1::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut c1 = result.c1;
            let result = match part_2::parse(state, tracer.clone(), cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut c2 = result.c2;
            Ok(ParseOk {
                result: Parsed { c1, c2 },
                state,
                farthest_error,
            })
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub c1: HexChar,
            pub c2: HexChar,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::HexaEscape> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|r| super::HexaEscape { c1: r.c1, c2: r.c2 }))
        }
    }
    #[inline]
    pub(super) fn parse_HexaEscape<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, HexaEscape> {
        tracer.run_traced("HexaEscape", state, |state, tracer| {
            HexaEscape_impl::rule_parser(state, tracer, cache)
        })
    }
    #[inline]
    pub(super) fn parse_HexChar<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, HexChar> {
        if let Ok(result) = parse_character_range(state.clone(), '0', '9') {
            return Ok(result);
        }
        if let Ok(result) = parse_character_range(state.clone(), 'a', 'f') {
            return Ok(result);
        }
        if let Ok(result) = parse_character_range(state.clone(), 'A', 'F') {
            return Ok(result);
        }
        Err(state.report_error(ParseErrorSpecifics::ExpectedCharacterClass { name: "HexChar" }))
    }
    mod Utf8Escape_impl {
        use super::*;
        mod choice_0 {
            use super::*;
            mod part_0 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_character_literal(state, 'u')?;
                    Ok(ok_result.map(|_| Parsed))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed;
            }
            mod part_1 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_character_literal(state, '{')?;
                    Ok(ok_result.map(|_| Parsed))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed;
            }
            mod part_2 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_HexChar(state, tracer, cache)?;
                    Ok(ok_result.map(|result| Parsed { c1: result }))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub c1: HexChar,
                }
            }
            mod part_3 {
                use super::*;
                mod optional {
                    use super::*;
                    mod part_0 {
                        use super::*;
                        #[inline(always)]
                        pub fn parse<'a>(
                            state: ParseState<'a>,
                            tracer: impl ParseTracer,
                            cache: &mut ParseCache<'a>,
                        ) -> ParseResult<'a, Parsed> {
                            let ok_result = parse_HexChar(state, tracer, cache)?;
                            Ok(ok_result.map(|result| Parsed { c2: Some(result) }))
                        }
                        #[derive(Debug, Clone, PartialEq, Eq)]
                        pub struct Parsed {
                            pub c2: Option<HexChar>,
                        }
                    }
                    mod part_1 {
                        use super::*;
                        mod optional {
                            use super::*;
                            mod part_0 {
                                use super::*;
                                #[inline(always)]
                                pub fn parse<'a>(
                                    state: ParseState<'a>,
                                    tracer: impl ParseTracer,
                                    cache: &mut ParseCache<'a>,
                                ) -> ParseResult<'a, Parsed> {
                                    let ok_result = parse_HexChar(state, tracer, cache)?;
                                    Ok(ok_result.map(|result| Parsed { c3: Some(result) }))
                                }
                                #[derive(Debug, Clone, PartialEq, Eq)]
                                pub struct Parsed {
                                    pub c3: Option<HexChar>,
                                }
                            }
                            mod part_1 {
                                use super::*;
                                mod optional {
                                    use super::*;
                                    mod part_0 {
                                        use super::*;
                                        #[inline(always)]
                                        pub fn parse<'a>(
                                            state: ParseState<'a>,
                                            tracer: impl ParseTracer,
                                            cache: &mut ParseCache<'a>,
                                        ) -> ParseResult<'a, Parsed>
                                        {
                                            let ok_result = parse_HexChar(state, tracer, cache)?;
                                            Ok(ok_result.map(|result| Parsed { c4: Some(result) }))
                                        }
                                        #[derive(Debug, Clone, PartialEq, Eq)]
                                        pub struct Parsed {
                                            pub c4: Option<HexChar>,
                                        }
                                    }
                                    mod part_1 {
                                        use super::*;
                                        mod optional {
                                            use super::*;
                                            mod part_0 {
                                                use super::*;
                                                #[inline(always)]
                                                pub fn parse<'a>(
                                                    state: ParseState<'a>,
                                                    tracer: impl ParseTracer,
                                                    cache: &mut ParseCache<'a>,
                                                ) -> ParseResult<'a, Parsed>
                                                {
                                                    let ok_result =
                                                        parse_HexChar(state, tracer, cache)?;
                                                    Ok(ok_result
                                                        .map(|result| Parsed { c5: Some(result) }))
                                                }
                                                #[derive(Debug, Clone, PartialEq, Eq)]
                                                pub struct Parsed {
                                                    pub c5: Option<HexChar>,
                                                }
                                            }
                                            mod part_1 {
                                                use super::*;
                                                mod optional {
                                                    use super::*;
                                                    #[inline(always)]
                                                    pub fn parse<'a>(
                                                        state: ParseState<'a>,
                                                        tracer: impl ParseTracer,
                                                        cache: &mut ParseCache<'a>,
                                                    ) -> ParseResult<'a, Parsed>
                                                    {
                                                        let ok_result =
                                                            parse_HexChar(state, tracer, cache)?;
                                                        Ok(ok_result.map(|result| Parsed {
                                                            c6: Some(result),
                                                        }))
                                                    }
                                                    #[derive(Debug, Clone, PartialEq, Eq)]
                                                    pub struct Parsed {
                                                        pub c6: Option<HexChar>,
                                                    }
                                                }
                                                #[inline(always)]
                                                pub fn parse<'a>(
                                                    state: ParseState<'a>,
                                                    tracer: impl ParseTracer,
                                                    cache: &mut ParseCache<'a>,
                                                ) -> ParseResult<'a, Parsed>
                                                {
                                                    match optional::parse(
                                                        state.clone(),
                                                        tracer,
                                                        cache,
                                                    ) {
                                                        Ok(ok_result) => {
                                                            Ok(ok_result.map(|result| Parsed {
                                                                c6: result.c6,
                                                            }))
                                                        }
                                                        Err(err) => Ok(ParseOk {
                                                            result: Parsed {
                                                                c6: Default::default(),
                                                            },
                                                            state,
                                                            farthest_error: Some(err),
                                                        }),
                                                    }
                                                }
                                                #[derive(Debug, Clone, PartialEq, Eq)]
                                                pub struct Parsed {
                                                    pub c6: Option<HexChar>,
                                                }
                                            }
                                            #[inline(always)]
                                            pub fn parse<'a>(
                                                state: ParseState<'a>,
                                                tracer: impl ParseTracer,
                                                cache: &mut ParseCache<'a>,
                                            ) -> ParseResult<'a, Parsed>
                                            {
                                                let mut state = state;
                                                let mut farthest_error: Option<ParseError> = None;
                                                let result = match part_0::parse(
                                                    state,
                                                    tracer.clone(),
                                                    cache,
                                                ) {
                                                    Ok(ParseOk {
                                                        result,
                                                        state: new_state,
                                                        farthest_error: new_farthest_error,
                                                    }) => {
                                                        farthest_error = combine_errors(
                                                            farthest_error,
                                                            new_farthest_error,
                                                        );
                                                        state = new_state;
                                                        result
                                                    }
                                                    Err(err) => {
                                                        return Err(combine_errors(
                                                            farthest_error,
                                                            Some(err),
                                                        )
                                                        .unwrap())
                                                    }
                                                };
                                                let mut c5 = result.c5;
                                                let result = match part_1::parse(
                                                    state,
                                                    tracer.clone(),
                                                    cache,
                                                ) {
                                                    Ok(ParseOk {
                                                        result,
                                                        state: new_state,
                                                        farthest_error: new_farthest_error,
                                                    }) => {
                                                        farthest_error = combine_errors(
                                                            farthest_error,
                                                            new_farthest_error,
                                                        );
                                                        state = new_state;
                                                        result
                                                    }
                                                    Err(err) => {
                                                        return Err(combine_errors(
                                                            farthest_error,
                                                            Some(err),
                                                        )
                                                        .unwrap())
                                                    }
                                                };
                                                let mut c6 = result.c6;
                                                Ok(ParseOk {
                                                    result: Parsed { c5, c6 },
                                                    state,
                                                    farthest_error,
                                                })
                                            }
                                            #[derive(Debug, Clone, PartialEq, Eq)]
                                            pub struct Parsed {
                                                pub c5: Option<HexChar>,
                                                pub c6: Option<HexChar>,
                                            }
                                        }
                                        #[inline(always)]
                                        pub fn parse<'a>(
                                            state: ParseState<'a>,
                                            tracer: impl ParseTracer,
                                            cache: &mut ParseCache<'a>,
                                        ) -> ParseResult<'a, Parsed>
                                        {
                                            match optional::parse(state.clone(), tracer, cache) {
                                                Ok(ok_result) => {
                                                    Ok(ok_result.map(|result| Parsed {
                                                        c5: result.c5,
                                                        c6: result.c6,
                                                    }))
                                                }
                                                Err(err) => Ok(ParseOk {
                                                    result: Parsed {
                                                        c5: Default::default(),
                                                        c6: Default::default(),
                                                    },
                                                    state,
                                                    farthest_error: Some(err),
                                                }),
                                            }
                                        }
                                        #[derive(Debug, Clone, PartialEq, Eq)]
                                        pub struct Parsed {
                                            pub c5: Option<HexChar>,
                                            pub c6: Option<HexChar>,
                                        }
                                    }
                                    #[inline(always)]
                                    pub fn parse<'a>(
                                        state: ParseState<'a>,
                                        tracer: impl ParseTracer,
                                        cache: &mut ParseCache<'a>,
                                    ) -> ParseResult<'a, Parsed>
                                    {
                                        let mut state = state;
                                        let mut farthest_error: Option<ParseError> = None;
                                        let result =
                                            match part_0::parse(state, tracer.clone(), cache) {
                                                Ok(ParseOk {
                                                    result,
                                                    state: new_state,
                                                    farthest_error: new_farthest_error,
                                                }) => {
                                                    farthest_error = combine_errors(
                                                        farthest_error,
                                                        new_farthest_error,
                                                    );
                                                    state = new_state;
                                                    result
                                                }
                                                Err(err) => {
                                                    return Err(combine_errors(
                                                        farthest_error,
                                                        Some(err),
                                                    )
                                                    .unwrap())
                                                }
                                            };
                                        let mut c4 = result.c4;
                                        let result =
                                            match part_1::parse(state, tracer.clone(), cache) {
                                                Ok(ParseOk {
                                                    result,
                                                    state: new_state,
                                                    farthest_error: new_farthest_error,
                                                }) => {
                                                    farthest_error = combine_errors(
                                                        farthest_error,
                                                        new_farthest_error,
                                                    );
                                                    state = new_state;
                                                    result
                                                }
                                                Err(err) => {
                                                    return Err(combine_errors(
                                                        farthest_error,
                                                        Some(err),
                                                    )
                                                    .unwrap())
                                                }
                                            };
                                        let mut c5 = result.c5;
                                        let mut c6 = result.c6;
                                        Ok(ParseOk {
                                            result: Parsed { c4, c5, c6 },
                                            state,
                                            farthest_error,
                                        })
                                    }
                                    #[derive(Debug, Clone, PartialEq, Eq)]
                                    pub struct Parsed {
                                        pub c4: Option<HexChar>,
                                        pub c5: Option<HexChar>,
                                        pub c6: Option<HexChar>,
                                    }
                                }
                                #[inline(always)]
                                pub fn parse<'a>(
                                    state: ParseState<'a>,
                                    tracer: impl ParseTracer,
                                    cache: &mut ParseCache<'a>,
                                ) -> ParseResult<'a, Parsed> {
                                    match optional::parse(state.clone(), tracer, cache) {
                                        Ok(ok_result) => Ok(ok_result.map(|result| Parsed {
                                            c4: result.c4,
                                            c5: result.c5,
                                            c6: result.c6,
                                        })),
                                        Err(err) => Ok(ParseOk {
                                            result: Parsed {
                                                c4: Default::default(),
                                                c5: Default::default(),
                                                c6: Default::default(),
                                            },
                                            state,
                                            farthest_error: Some(err),
                                        }),
                                    }
                                }
                                #[derive(Debug, Clone, PartialEq, Eq)]
                                pub struct Parsed {
                                    pub c4: Option<HexChar>,
                                    pub c5: Option<HexChar>,
                                    pub c6: Option<HexChar>,
                                }
                            }
                            #[inline(always)]
                            pub fn parse<'a>(
                                state: ParseState<'a>,
                                tracer: impl ParseTracer,
                                cache: &mut ParseCache<'a>,
                            ) -> ParseResult<'a, Parsed> {
                                let mut state = state;
                                let mut farthest_error: Option<ParseError> = None;
                                let result = match part_0::parse(state, tracer.clone(), cache) {
                                    Ok(ParseOk {
                                        result,
                                        state: new_state,
                                        farthest_error: new_farthest_error,
                                    }) => {
                                        farthest_error =
                                            combine_errors(farthest_error, new_farthest_error);
                                        state = new_state;
                                        result
                                    }
                                    Err(err) => {
                                        return Err(
                                            combine_errors(farthest_error, Some(err)).unwrap()
                                        )
                                    }
                                };
                                let mut c3 = result.c3;
                                let result = match part_1::parse(state, tracer.clone(), cache) {
                                    Ok(ParseOk {
                                        result,
                                        state: new_state,
                                        farthest_error: new_farthest_error,
                                    }) => {
                                        farthest_error =
                                            combine_errors(farthest_error, new_farthest_error);
                                        state = new_state;
                                        result
                                    }
                                    Err(err) => {
                                        return Err(
                                            combine_errors(farthest_error, Some(err)).unwrap()
                                        )
                                    }
                                };
                                let mut c4 = result.c4;
                                let mut c5 = result.c5;
                                let mut c6 = result.c6;
                                Ok(ParseOk {
                                    result: Parsed { c3, c4, c5, c6 },
                                    state,
                                    farthest_error,
                                })
                            }
                            #[derive(Debug, Clone, PartialEq, Eq)]
                            pub struct Parsed {
                                pub c3: Option<HexChar>,
                                pub c4: Option<HexChar>,
                                pub c5: Option<HexChar>,
                                pub c6: Option<HexChar>,
                            }
                        }
                        #[inline(always)]
                        pub fn parse<'a>(
                            state: ParseState<'a>,
                            tracer: impl ParseTracer,
                            cache: &mut ParseCache<'a>,
                        ) -> ParseResult<'a, Parsed> {
                            match optional::parse(state.clone(), tracer, cache) {
                                Ok(ok_result) => Ok(ok_result.map(|result| Parsed {
                                    c3: result.c3,
                                    c4: result.c4,
                                    c5: result.c5,
                                    c6: result.c6,
                                })),
                                Err(err) => Ok(ParseOk {
                                    result: Parsed {
                                        c3: Default::default(),
                                        c4: Default::default(),
                                        c5: Default::default(),
                                        c6: Default::default(),
                                    },
                                    state,
                                    farthest_error: Some(err),
                                }),
                            }
                        }
                        #[derive(Debug, Clone, PartialEq, Eq)]
                        pub struct Parsed {
                            pub c3: Option<HexChar>,
                            pub c4: Option<HexChar>,
                            pub c5: Option<HexChar>,
                            pub c6: Option<HexChar>,
                        }
                    }
                    #[inline(always)]
                    pub fn parse<'a>(
                        state: ParseState<'a>,
                        tracer: impl ParseTracer,
                        cache: &mut ParseCache<'a>,
                    ) -> ParseResult<'a, Parsed> {
                        let mut state = state;
                        let mut farthest_error: Option<ParseError> = None;
                        let result = match part_0::parse(state, tracer.clone(), cache) {
                            Ok(ParseOk {
                                result,
                                state: new_state,
                                farthest_error: new_farthest_error,
                            }) => {
                                farthest_error = combine_errors(farthest_error, new_farthest_error);
                                state = new_state;
                                result
                            }
                            Err(err) => {
                                return Err(combine_errors(farthest_error, Some(err)).unwrap())
                            }
                        };
                        let mut c2 = result.c2;
                        let result = match part_1::parse(state, tracer.clone(), cache) {
                            Ok(ParseOk {
                                result,
                                state: new_state,
                                farthest_error: new_farthest_error,
                            }) => {
                                farthest_error = combine_errors(farthest_error, new_farthest_error);
                                state = new_state;
                                result
                            }
                            Err(err) => {
                                return Err(combine_errors(farthest_error, Some(err)).unwrap())
                            }
                        };
                        let mut c3 = result.c3;
                        let mut c4 = result.c4;
                        let mut c5 = result.c5;
                        let mut c6 = result.c6;
                        Ok(ParseOk {
                            result: Parsed { c2, c3, c4, c5, c6 },
                            state,
                            farthest_error,
                        })
                    }
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct Parsed {
                        pub c2: Option<HexChar>,
                        pub c3: Option<HexChar>,
                        pub c4: Option<HexChar>,
                        pub c5: Option<HexChar>,
                        pub c6: Option<HexChar>,
                    }
                }
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    match optional::parse(state.clone(), tracer, cache) {
                        Ok(ok_result) => Ok(ok_result.map(|result| Parsed {
                            c2: result.c2,
                            c3: result.c3,
                            c4: result.c4,
                            c5: result.c5,
                            c6: result.c6,
                        })),
                        Err(err) => Ok(ParseOk {
                            result: Parsed {
                                c2: Default::default(),
                                c3: Default::default(),
                                c4: Default::default(),
                                c5: Default::default(),
                                c6: Default::default(),
                            },
                            state,
                            farthest_error: Some(err),
                        }),
                    }
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub c2: Option<HexChar>,
                    pub c3: Option<HexChar>,
                    pub c4: Option<HexChar>,
                    pub c5: Option<HexChar>,
                    pub c6: Option<HexChar>,
                }
            }
            mod part_4 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_character_literal(state, '}')?;
                    Ok(ok_result.map(|_| Parsed))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed;
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let mut state = state;
                let mut farthest_error: Option<ParseError> = None;
                match part_0::parse(state, tracer.clone(), cache) {
                    Ok(ParseOk {
                        result: _,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                        state = new_state;
                    }
                    Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                }
                match part_1::parse(state, tracer.clone(), cache) {
                    Ok(ParseOk {
                        result: _,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                        state = new_state;
                    }
                    Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                }
                let result = match part_2::parse(state, tracer.clone(), cache) {
                    Ok(ParseOk {
                        result,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                        state = new_state;
                        result
                    }
                    Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                };
                let mut c1 = result.c1;
                let result = match part_3::parse(state, tracer.clone(), cache) {
                    Ok(ParseOk {
                        result,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                        state = new_state;
                        result
                    }
                    Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                };
                let mut c2 = result.c2;
                let mut c3 = result.c3;
                let mut c4 = result.c4;
                let mut c5 = result.c5;
                let mut c6 = result.c6;
                match part_4::parse(state, tracer.clone(), cache) {
                    Ok(ParseOk {
                        result: _,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                        state = new_state;
                    }
                    Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                }
                Ok(ParseOk {
                    result: Parsed {
                        c1,
                        c2,
                        c3,
                        c4,
                        c5,
                        c6,
                    },
                    state,
                    farthest_error,
                })
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub c1: HexChar,
                pub c2: Option<HexChar>,
                pub c3: Option<HexChar>,
                pub c4: Option<HexChar>,
                pub c5: Option<HexChar>,
                pub c6: Option<HexChar>,
            }
        }
        mod choice_1 {
            use super::*;
            mod part_0 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_character_literal(state, 'u')?;
                    Ok(ok_result.map(|_| Parsed))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed;
            }
            mod part_1 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_HexChar(state, tracer, cache)?;
                    Ok(ok_result.map(|result| Parsed { c1: result }))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub c1: HexChar,
                }
            }
            mod part_2 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_HexChar(state, tracer, cache)?;
                    Ok(ok_result.map(|result| Parsed { c2: Some(result) }))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub c2: Option<HexChar>,
                }
            }
            mod part_3 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_HexChar(state, tracer, cache)?;
                    Ok(ok_result.map(|result| Parsed { c3: Some(result) }))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub c3: Option<HexChar>,
                }
            }
            mod part_4 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_HexChar(state, tracer, cache)?;
                    Ok(ok_result.map(|result| Parsed { c4: Some(result) }))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub c4: Option<HexChar>,
                }
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let mut state = state;
                let mut farthest_error: Option<ParseError> = None;
                match part_0::parse(state, tracer.clone(), cache) {
                    Ok(ParseOk {
                        result: _,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                        state = new_state;
                    }
                    Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                }
                let result = match part_1::parse(state, tracer.clone(), cache) {
                    Ok(ParseOk {
                        result,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                        state = new_state;
                        result
                    }
                    Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                };
                let mut c1 = result.c1;
                let result = match part_2::parse(state, tracer.clone(), cache) {
                    Ok(ParseOk {
                        result,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                        state = new_state;
                        result
                    }
                    Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                };
                let mut c2 = result.c2;
                let result = match part_3::parse(state, tracer.clone(), cache) {
                    Ok(ParseOk {
                        result,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                        state = new_state;
                        result
                    }
                    Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                };
                let mut c3 = result.c3;
                let result = match part_4::parse(state, tracer.clone(), cache) {
                    Ok(ParseOk {
                        result,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                        state = new_state;
                        result
                    }
                    Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                };
                let mut c4 = result.c4;
                Ok(ParseOk {
                    result: Parsed { c1, c2, c3, c4 },
                    state,
                    farthest_error,
                })
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub c1: HexChar,
                pub c2: Option<HexChar>,
                pub c3: Option<HexChar>,
                pub c4: Option<HexChar>,
            }
        }
        mod choice_2 {
            use super::*;
            mod part_0 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_character_literal(state, 'U')?;
                    Ok(ok_result.map(|_| Parsed))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed;
            }
            mod part_1 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_character_literal(state, '0')?;
                    Ok(ok_result.map(|_| Parsed))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed;
            }
            mod part_2 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_character_literal(state, '0')?;
                    Ok(ok_result.map(|_| Parsed))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed;
            }
            mod part_3 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_HexChar(state, tracer, cache)?;
                    Ok(ok_result.map(|result| Parsed { c1: result }))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub c1: HexChar,
                }
            }
            mod part_4 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_HexChar(state, tracer, cache)?;
                    Ok(ok_result.map(|result| Parsed { c2: Some(result) }))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub c2: Option<HexChar>,
                }
            }
            mod part_5 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_HexChar(state, tracer, cache)?;
                    Ok(ok_result.map(|result| Parsed { c3: Some(result) }))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub c3: Option<HexChar>,
                }
            }
            mod part_6 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_HexChar(state, tracer, cache)?;
                    Ok(ok_result.map(|result| Parsed { c4: Some(result) }))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub c4: Option<HexChar>,
                }
            }
            mod part_7 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_HexChar(state, tracer, cache)?;
                    Ok(ok_result.map(|result| Parsed { c5: Some(result) }))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub c5: Option<HexChar>,
                }
            }
            mod part_8 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_HexChar(state, tracer, cache)?;
                    Ok(ok_result.map(|result| Parsed { c6: Some(result) }))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub c6: Option<HexChar>,
                }
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let mut state = state;
                let mut farthest_error: Option<ParseError> = None;
                match part_0::parse(state, tracer.clone(), cache) {
                    Ok(ParseOk {
                        result: _,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                        state = new_state;
                    }
                    Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                }
                match part_1::parse(state, tracer.clone(), cache) {
                    Ok(ParseOk {
                        result: _,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                        state = new_state;
                    }
                    Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                }
                match part_2::parse(state, tracer.clone(), cache) {
                    Ok(ParseOk {
                        result: _,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                        state = new_state;
                    }
                    Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                }
                let result = match part_3::parse(state, tracer.clone(), cache) {
                    Ok(ParseOk {
                        result,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                        state = new_state;
                        result
                    }
                    Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                };
                let mut c1 = result.c1;
                let result = match part_4::parse(state, tracer.clone(), cache) {
                    Ok(ParseOk {
                        result,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                        state = new_state;
                        result
                    }
                    Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                };
                let mut c2 = result.c2;
                let result = match part_5::parse(state, tracer.clone(), cache) {
                    Ok(ParseOk {
                        result,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                        state = new_state;
                        result
                    }
                    Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                };
                let mut c3 = result.c3;
                let result = match part_6::parse(state, tracer.clone(), cache) {
                    Ok(ParseOk {
                        result,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                        state = new_state;
                        result
                    }
                    Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                };
                let mut c4 = result.c4;
                let result = match part_7::parse(state, tracer.clone(), cache) {
                    Ok(ParseOk {
                        result,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                        state = new_state;
                        result
                    }
                    Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                };
                let mut c5 = result.c5;
                let result = match part_8::parse(state, tracer.clone(), cache) {
                    Ok(ParseOk {
                        result,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                        state = new_state;
                        result
                    }
                    Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                };
                let mut c6 = result.c6;
                Ok(ParseOk {
                    result: Parsed {
                        c1,
                        c2,
                        c3,
                        c4,
                        c5,
                        c6,
                    },
                    state,
                    farthest_error,
                })
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub c1: HexChar,
                pub c2: Option<HexChar>,
                pub c3: Option<HexChar>,
                pub c4: Option<HexChar>,
                pub c5: Option<HexChar>,
                pub c6: Option<HexChar>,
            }
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            match choice_0::parse(state.clone(), tracer.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        c1: result.c1,
                        c2: result.c2,
                        c3: result.c3,
                        c4: result.c4,
                        c5: result.c5,
                        c6: result.c6,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_1::parse(state.clone(), tracer.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        c1: result.c1,
                        c2: result.c2,
                        c3: result.c3,
                        c4: result.c4,
                        c5: None,
                        c6: None,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_2::parse(state.clone(), tracer.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        c1: result.c1,
                        c2: result.c2,
                        c3: result.c3,
                        c4: result.c4,
                        c5: result.c5,
                        c6: result.c6,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            Err(farthest_error.unwrap_or_else(|| state.report_error(ParseErrorSpecifics::Other)))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub c1: HexChar,
            pub c2: Option<HexChar>,
            pub c3: Option<HexChar>,
            pub c4: Option<HexChar>,
            pub c5: Option<HexChar>,
            pub c6: Option<HexChar>,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::Utf8Escape> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|r| super::Utf8Escape {
                c1: r.c1,
                c2: r.c2,
                c3: r.c3,
                c4: r.c4,
                c5: r.c5,
                c6: r.c6,
            }))
        }
    }
    #[inline]
    pub(super) fn parse_Utf8Escape<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, Utf8Escape> {
        tracer.run_traced("Utf8Escape", state, |state, tracer| {
            Utf8Escape_impl::rule_parser(state, tracer, cache)
        })
    }
    mod DirectiveExpression_impl {
        use super::*;
        mod choice_0 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_StringDirective(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::StringDirective(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_1 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_NoSkipWsDirective(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::NoSkipWsDirective(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_2 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_ExportDirective(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::ExportDirective(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_3 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_PositionDirective(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::PositionDirective(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_4 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_MemoizeDirective(state, tracer, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::MemoizeDirective(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            match choice_0::parse(state.clone(), tracer.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_1::parse(state.clone(), tracer.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_2::parse(state.clone(), tracer.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_3::parse(state.clone(), tracer.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_4::parse(state.clone(), tracer.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            Err(farthest_error.unwrap_or_else(|| state.report_error(ParseErrorSpecifics::Other)))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub _override: Parsed__override,
        }
        use super::DirectiveExpression as Parsed__override;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::DirectiveExpression> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|result| result._override))
        }
    }
    #[inline]
    pub(super) fn parse_DirectiveExpression<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, DirectiveExpression> {
        tracer.run_traced("DirectiveExpression", state, |state, tracer| {
            DirectiveExpression_impl::rule_parser(state, tracer, cache)
        })
    }
    mod StringDirective_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let state = state.skip_whitespace();
            let ok_result = parse_string_literal(state, "@string")?;
            Ok(ok_result.map(|_| Parsed))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::StringDirective> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|r| super::StringDirective {}))
        }
    }
    #[inline]
    pub(super) fn parse_StringDirective<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, StringDirective> {
        tracer.run_traced("StringDirective", state, |state, tracer| {
            StringDirective_impl::rule_parser(state, tracer, cache)
        })
    }
    mod CharDirective_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let state = state.skip_whitespace();
            let ok_result = parse_string_literal(state, "@char")?;
            Ok(ok_result.map(|_| Parsed))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::CharDirective> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|r| super::CharDirective {}))
        }
    }
    #[inline]
    pub(super) fn parse_CharDirective<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, CharDirective> {
        tracer.run_traced("CharDirective", state, |state, tracer| {
            CharDirective_impl::rule_parser(state, tracer, cache)
        })
    }
    mod NoSkipWsDirective_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let state = state.skip_whitespace();
            let ok_result = parse_string_literal(state, "@no_skip_ws")?;
            Ok(ok_result.map(|_| Parsed))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::NoSkipWsDirective> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|r| super::NoSkipWsDirective {}))
        }
    }
    #[inline]
    pub(super) fn parse_NoSkipWsDirective<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, NoSkipWsDirective> {
        tracer.run_traced("NoSkipWsDirective", state, |state, tracer| {
            NoSkipWsDirective_impl::rule_parser(state, tracer, cache)
        })
    }
    mod ExportDirective_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let state = state.skip_whitespace();
            let ok_result = parse_string_literal(state, "@export")?;
            Ok(ok_result.map(|_| Parsed))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::ExportDirective> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|r| super::ExportDirective {}))
        }
    }
    #[inline]
    pub(super) fn parse_ExportDirective<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, ExportDirective> {
        tracer.run_traced("ExportDirective", state, |state, tracer| {
            ExportDirective_impl::rule_parser(state, tracer, cache)
        })
    }
    mod PositionDirective_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let state = state.skip_whitespace();
            let ok_result = parse_string_literal(state, "@position")?;
            Ok(ok_result.map(|_| Parsed))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::PositionDirective> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|r| super::PositionDirective {}))
        }
    }
    #[inline]
    pub(super) fn parse_PositionDirective<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, PositionDirective> {
        tracer.run_traced("PositionDirective", state, |state, tracer| {
            PositionDirective_impl::rule_parser(state, tracer, cache)
        })
    }
    mod MemoizeDirective_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let state = state.skip_whitespace();
            let ok_result = parse_string_literal(state, "@memoize")?;
            Ok(ok_result.map(|_| Parsed))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::MemoizeDirective> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|r| super::MemoizeDirective {}))
        }
    }
    #[inline]
    pub(super) fn parse_MemoizeDirective<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, MemoizeDirective> {
        tracer.run_traced("MemoizeDirective", state, |state, tracer| {
            MemoizeDirective_impl::rule_parser(state, tracer, cache)
        })
    }
    mod EndOfInput_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let state = state.skip_whitespace();
            let ok_result = parse_character_literal(state, '$')?;
            Ok(ok_result.map(|_| Parsed))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::EndOfInput> {
            let ok_result = parse(state, tracer, cache)?;
            Ok(ok_result.map(|r| super::EndOfInput {}))
        }
    }
    #[inline]
    pub(super) fn parse_EndOfInput<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, EndOfInput> {
        tracer.run_traced("EndOfInput", state, |state, tracer| {
            EndOfInput_impl::rule_parser(state, tracer, cache)
        })
    }
}
