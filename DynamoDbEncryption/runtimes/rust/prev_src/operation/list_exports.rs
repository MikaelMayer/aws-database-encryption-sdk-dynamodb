// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `ListExports`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListExports;
impl ListExports {
    /// Creates a new `ListExports`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::list_exports::ListExportsInput,
    ) -> ::std::result::Result<
        crate::operation::list_exports::ListExportsOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::list_exports::_list_exports_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).ListExports(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::list_exports::_list_exports_output::from_dafny(
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

pub use crate::operation::list_exports::_list_exports_output::ListExportsOutput;

pub use crate::operation::list_exports::_list_exports_input::ListExportsInput;

pub(crate) mod _list_exports_output;

pub(crate) mod _list_exports_input;

/// Builders
pub mod builders;
