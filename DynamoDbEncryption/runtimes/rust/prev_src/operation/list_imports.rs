// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `ListImports`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListImports;
impl ListImports {
    /// Creates a new `ListImports`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::list_imports::ListImportsInput,
    ) -> ::std::result::Result<
        crate::operation::list_imports::ListImportsOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::list_imports::_list_imports_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).ListImports(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::list_imports::_list_imports_output::from_dafny(
                    inner_result.value().clone(),
                ),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::list_imports::_list_imports_output::ListImportsOutput;

pub use crate::operation::list_imports::_list_imports_input::ListImportsInput;

pub(crate) mod _list_imports_output;

pub(crate) mod _list_imports_input;

/// Builders
pub mod builders;
