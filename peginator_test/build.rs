// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

fn main() {
    peginator::buildscript::Compile::directory("src")
        .format()
        .use_peginator_build_time()
        .derives(vec![
            "Debug".into(),
            "Clone".into(),
            "PartialEq".into(),
            "Eq".into(),
        ])
        .run_exit_on_error();
    peginator::buildscript::Compile::file("src/custom_derives_empty/grammar.not_ebnf")
        .format()
        .use_peginator_build_time()
        .derives(vec![])
        .prefix("pub struct ImJustHereToConfuse;".into())
        .run_exit_on_error();
    peginator::buildscript::Compile::file("src/user_defined_state/grammar.not_ebnf")
        .format()
        .use_peginator_build_time()
        .derives(vec![
            "Debug".into(),
            "Clone".into(),
            "PartialEq".into(),
            "Eq".into(),
        ])
        .user_defined_type("crate::user_defined_state::TheState")
        .run_exit_on_error();
}
