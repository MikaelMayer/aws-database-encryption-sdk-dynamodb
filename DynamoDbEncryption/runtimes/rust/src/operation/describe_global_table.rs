// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `DescribeGlobalTable`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DescribeGlobalTable;
impl DescribeGlobalTable {
    /// Creates a new `DescribeGlobalTable`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::describe_global_table::DescribeGlobalTableInput,
    ) -> ::std::result::Result<
        crate::operation::describe_global_table::DescribeGlobalTableOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::describe_global_table::_describe_global_table_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).DescribeGlobalTable(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::describe_global_table::_describe_global_table_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::describe_global_table::_describe_global_table_output::DescribeGlobalTableOutput;

pub use crate::operation::describe_global_table::_describe_global_table_input::DescribeGlobalTableInput;

pub(crate) mod _describe_global_table_output;

pub(crate) mod _describe_global_table_input;

/// Builders
pub mod builders;
