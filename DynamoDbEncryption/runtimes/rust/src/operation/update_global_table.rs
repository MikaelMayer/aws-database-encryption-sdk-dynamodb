// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `UpdateGlobalTable`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UpdateGlobalTable;
impl UpdateGlobalTable {
    /// Creates a new `UpdateGlobalTable`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::update_global_table::UpdateGlobalTableInput,
    ) -> ::std::result::Result<
        crate::operation::update_global_table::UpdateGlobalTableOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::update_global_table::_update_global_table_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).UpdateGlobalTable(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::update_global_table::_update_global_table_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::update_global_table::_update_global_table_output::UpdateGlobalTableOutput;

pub use crate::operation::update_global_table::_update_global_table_input::UpdateGlobalTableInput;

pub(crate) mod _update_global_table_output;

pub(crate) mod _update_global_table_input;

/// Builders
pub mod builders;
