// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `DescribeContinuousBackups`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DescribeContinuousBackups;
impl DescribeContinuousBackups {
    /// Creates a new `DescribeContinuousBackups`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::describe_continuous_backups::DescribeContinuousBackupsInput,
    ) -> ::std::result::Result<
        crate::operation::describe_continuous_backups::DescribeContinuousBackupsOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::describe_continuous_backups::_describe_continuous_backups_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).DescribeContinuousBackups(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::describe_continuous_backups::_describe_continuous_backups_output::from_dafny(
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

pub use crate::operation::describe_continuous_backups::_describe_continuous_backups_output::DescribeContinuousBackupsOutput;

pub use crate::operation::describe_continuous_backups::_describe_continuous_backups_input::DescribeContinuousBackupsInput;

pub(crate) mod _describe_continuous_backups_output;

pub(crate) mod _describe_continuous_backups_input;

/// Builders
pub mod builders;
