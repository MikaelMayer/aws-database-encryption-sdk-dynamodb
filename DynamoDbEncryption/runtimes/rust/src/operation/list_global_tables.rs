// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `ListGlobalTables`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListGlobalTables;
impl ListGlobalTables {
    /// Creates a new `ListGlobalTables`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::list_global_tables::ListGlobalTablesInput,
    ) -> ::std::result::Result<
        crate::operation::list_global_tables::ListGlobalTablesOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::list_global_tables::_list_global_tables_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).ListGlobalTables(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::list_global_tables::_list_global_tables_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::list_global_tables::_list_global_tables_output::ListGlobalTablesOutput;

pub use crate::operation::list_global_tables::_list_global_tables_input::ListGlobalTablesInput;

pub(crate) mod _list_global_tables_output;

pub(crate) mod _list_global_tables_input;

/// Builders
pub mod builders;
