/*
 * Copyright (c) 2022, JT <jt@serenityos.org>
 * Copyright (c) 2022, Andreas Kling <kling@serenityos.org>
 *
 * SPDX-License-Identifier: BSD-2-Clause
 */

#![deny(clippy::all)]
#![allow(clippy::or_fun_call)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::never_loop)]

extern crate core;

mod codegen;
mod compiler;
mod error;
mod ide;
mod lexer;
mod parser;
mod typechecker;

pub use compiler::Compiler;
pub use error::JaktError;
pub use ide::{
    find_definition_in_project, find_dot_completions_in_project, find_typename_in_project,
};
pub use lexer::Span;
pub use typechecker::Project;
