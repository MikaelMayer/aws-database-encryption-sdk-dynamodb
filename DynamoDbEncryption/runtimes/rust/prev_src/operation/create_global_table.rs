// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `CreateGlobalTable`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CreateGlobalTable;
impl CreateGlobalTable {
    /// Creates a new `CreateGlobalTable`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::create_global_table::CreateGlobalTableInput,
    ) -> ::std::result::Result<
        crate::operation::create_global_table::CreateGlobalTableOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::create_global_table::_create_global_table_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).CreateGlobalTable(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::create_global_table::_create_global_table_output::from_dafny(
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

pub use crate::operation::create_global_table::_create_global_table_output::CreateGlobalTableOutput;

pub use crate::operation::create_global_table::_create_global_table_input::CreateGlobalTableInput;

pub(crate) mod _create_global_table_output;

pub(crate) mod _create_global_table_input;

/// Builders
pub mod builders;
