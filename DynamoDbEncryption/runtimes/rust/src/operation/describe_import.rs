// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `DescribeImport`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DescribeImport;
impl DescribeImport {
    /// Creates a new `DescribeImport`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::describe_import::DescribeImportInput,
    ) -> ::std::result::Result<
        crate::operation::describe_import::DescribeImportOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::describe_import::_describe_import_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).DescribeImport(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::describe_import::_describe_import_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::describe_import::_describe_import_output::DescribeImportOutput;

pub use crate::operation::describe_import::_describe_import_input::DescribeImportInput;

pub(crate) mod _describe_import_output;

pub(crate) mod _describe_import_input;

/// Builders
pub mod builders;
