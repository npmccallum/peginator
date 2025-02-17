// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use sha2::{Digest, Sha256};

use super::{BUILD_TIME, VERSION};

pub fn generate_source_header(grammar: &str, use_build_time: bool) -> String {
    let version = if use_build_time {
        format!("{VERSION}@{BUILD_TIME}")
    } else {
        VERSION.to_string()
    };
    let hash = Sha256::digest(grammar);
    format!(
        "// This file was generated by Peginator v{version}\n\
         // Hash of the grammar file: {hash:X}\n\
         // Any changes to it will be lost on regeneration\n",
    )
}
