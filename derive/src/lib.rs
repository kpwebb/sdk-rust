// Copyright (c) 2023 -  Restate Software, Inc., Restate GmbH.
// All rights reserved.
//
// Use of this software is governed by the Business Source License
// included in the LICENSE file.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0.

//! Some parts of this codebase were taken from https://github.com/dtolnay/thiserror/tree/39aaeb00ff270a49e3c254d7b38b10e934d3c7a5/impl/src
//! License Apache-2.0 or MIT

extern crate proc_macro;

mod ast;
mod gen;

use crate::ast::Service;
use crate::gen::{ServiceGenerator, ServiceType};
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn service(_: TokenStream, input: TokenStream) -> TokenStream {
    let svc = parse_macro_input!(input as Service);

    ServiceGenerator::new(ServiceType::Service, &svc)
        .into_token_stream()
        .into()
}

#[proc_macro_attribute]
pub fn object(_: TokenStream, input: TokenStream) -> TokenStream {
    let svc = parse_macro_input!(input as Service);

    ServiceGenerator::new(ServiceType::VirtualObject, &svc)
        .into_token_stream()
        .into()
}

#[proc_macro_attribute]
pub fn workflow(_: TokenStream, input: TokenStream) -> TokenStream {
    let svc = parse_macro_input!(input as Service);

    ServiceGenerator::new(ServiceType::Workflow, &svc)
        .into_token_stream()
        .into()
}
